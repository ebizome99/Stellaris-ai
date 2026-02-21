//! GPU调度器模块
//! 
//! 多GPU资源管理和任务调度

use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use crate::core::types::{GPUId, GPUStatus};
use crate::core::error::{Error, Result};
use crate::core::config::AppConfig;
use crate::core::constants::limits::LOW_VRAM_THRESHOLD_MB;

/// GPU信息
#[derive(Debug, Clone)]
pub struct GPUInfo {
    /// GPU ID
    pub id: GPUId,
    /// GPU名称
    pub name: String,
    /// 总显存 (MB)
    pub total_memory_mb: u64,
    /// 计算能力
    pub compute_capability: (u32, u32),
}

/// GPU调度器
pub struct GPUScheduler {
    /// 应用配置
    config: Arc<RwLock<AppConfig>>,
    /// GPU列表
    gpus: RwLock<Vec<GPUInfo>>,
    /// GPU状态
    gpu_status: RwLock<Vec<GPUStatus>>,
    /// GPU任务分配
    gpu_tasks: Mutex<Vec<(GPUId, Vec<uuid::Uuid>)>>,
    /// 显存分配
    vram_allocation: RwLock<Vec<(GPUId, u64)>>,
}

impl GPUScheduler {
    /// 创建新的GPU调度器
    pub async fn new(config: Arc<RwLock<AppConfig>>) -> Result<Self> {
        let gpus = Self::detect_gpus().await?;
        
        let gpu_status: Vec<GPUStatus> = gpus.iter().map(|gpu| GPUStatus {
            id: gpu.id,
            name: gpu.name.clone(),
            total_memory_mb: gpu.total_memory_mb,
            used_memory_mb: 0,
            free_memory_mb: gpu.total_memory_mb,
            memory_utilization: 0.0,
            gpu_utilization: 0.0,
            temperature: None,
            active_tasks: 0,
            queue_length: 0,
            available: true,
        }).collect();
        
        let gpu_tasks = gpus.iter()
            .map(|gpu| (gpu.id, Vec::new()))
            .collect();
        
        let vram_allocation = gpus.iter()
            .map(|gpu| (gpu.id, 0u64))
            .collect();
        
        Ok(Self {
            config,
            gpus: RwLock::new(gpus),
            gpu_status: RwLock::new(gpu_status),
            gpu_tasks: Mutex::new(gpu_tasks),
            vram_allocation: RwLock::new(vram_allocation),
        })
    }
    
    /// 检测系统GPU
    async fn detect_gpus() -> Result<Vec<GPUInfo>> {
        let mut gpus = Vec::new();
        
        #[cfg(feature = "cuda")]
        {
            if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                let count = nvml.device_count().unwrap_or(0);
                
                for i in 0..count {
                    if let Ok(device) = nvml.device_by_index(i) {
                        let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
                        let memory_info = device.memory_info().ok();
                        let total_memory = memory_info
                            .map(|m| m.total / 1024 / 1024)
                            .unwrap_or(0);
                        
                        gpus.push(GPUInfo {
                            id: GPUId(i as usize),
                            name,
                            total_memory_mb: total_memory,
                            compute_capability: (8, 9),
                        });
                    }
                }
            }
        }
        
        if gpus.is_empty() {
            tracing::info!("未检测到GPU，使用模拟模式");
            gpus.push(GPUInfo {
                id: GPUId(0),
                name: "Simulated GPU".to_string(),
                total_memory_mb: 8192,
                compute_capability: (8, 6),
            });
        }
        
        tracing::info!("检测到 {} 个GPU", gpus.len());
        for gpu in &gpus {
            tracing::info!("  - {}: {} ({}MB)", gpu.id, gpu.name, gpu.total_memory_mb);
        }
        
        Ok(gpus)
    }
    
    /// 获取GPU数量
    pub fn gpu_count(&self) -> usize {
        self.gpus.read().len()
    }
    
    /// 是否有可用GPU
    pub fn has_available_gpu(&self) -> bool {
        self.gpu_status.read().iter().any(|s| s.available)
    }
    
    /// 获取主GPU状态
    pub fn get_primary_gpu_status(&self) -> Option<GPUStatus> {
        self.gpu_status.read().first().cloned()
    }
    
    /// 获取所有GPU状态
    pub fn get_all_gpu_status(&self) -> Vec<GPUStatus> {
        self.gpu_status.read().clone()
    }
    
    /// 分配GPU
    pub async fn allocate_gpu(&self, required_vram_mb: u64) -> Result<GPUId> {
        let config = self.config.read();
        let strategy = config.gpu.scheduling_strategy;
        drop(config);
        
        let gpu_id = match strategy {
            crate::core::config::SchedulingStrategy::RoundRobin => {
                self.allocate_round_robin()
            }
            crate::core::config::SchedulingStrategy::LeastLoad => {
                self.allocate_least_load()
            }
            crate::core::config::SchedulingStrategy::VRAMFirst => {
                self.allocate_vram_first(required_vram_mb)
            }
            crate::core::config::SchedulingStrategy::Manual => {
                self.allocate_manual()
            }
        };
        
        gpu_id.ok_or_else(|| Error::GPU("没有可用的GPU资源".into()))
    }
    
    /// 轮询调度
    fn allocate_round_robin(&self) -> Option<GPUId> {
        let status = self.gpu_status.read();
        status.iter()
            .find(|s| s.available && s.free_memory_mb > 1000)
            .map(|s| s.id)
    }
    
    /// 最小负载调度
    fn allocate_least_load(&self) -> Option<GPUId> {
        let status = self.gpu_status.read();
        status.iter()
            .filter(|s| s.available && s.free_memory_mb > 1000)
            .min_by(|a, b| a.active_tasks.cmp(&b.active_tasks))
            .map(|s| s.id)
    }
    
    /// 显存优先调度
    fn allocate_vram_first(&self, required: u64) -> Option<GPUId> {
        let status = self.gpu_status.read();
        status.iter()
            .filter(|s| s.available && s.free_memory_mb >= required)
            .max_by(|a, b| a.free_memory_mb.cmp(&b.free_memory_mb))
            .map(|s| s.id)
    }
    
    /// 手动绑定调度
    fn allocate_manual(&self) -> Option<GPUId> {
        let status = self.gpu_status.read();
        status.iter()
            .find(|s| s.available)
            .map(|s| s.id)
    }
    
    /// 释放GPU
    pub async fn release_gpu(&self, gpu_id: GPUId) {
        let mut status = self.gpu_status.write();
        if let Some(gpu) = status.iter_mut().find(|g| g.id == gpu_id) {
            gpu.active_tasks = gpu.active_tasks.saturating_sub(1);
            if gpu.active_tasks == 0 {
                gpu.available = true;
            }
        }
    }
    
    /// 更新GPU状态
    pub fn update_status(&self, gpu_id: GPUId, used_memory: u64) {
        let mut status = self.gpu_status.write();
        if let Some(gpu) = status.iter_mut().find(|g| g.id == gpu_id) {
            gpu.used_memory_mb = used_memory;
            gpu.free_memory_mb = gpu.total_memory_mb.saturating_sub(used_memory);
            gpu.memory_utilization = if gpu.total_memory_mb > 0 {
                used_memory as f32 / gpu.total_memory_mb as f32
            } else {
                0.0
            };
        }
    }
    
    /// 检查低显存模式
    pub fn is_low_vram_mode(&self) -> bool {
        let gpus = self.gpus.read();
        gpus.iter().any(|g| g.total_memory_mb <= LOW_VRAM_THRESHOLD_MB)
    }
    
    /// 获取总显存
    pub fn total_vram(&self) -> u64 {
        self.gpus.read().iter().map(|g| g.total_memory_mb).sum()
    }
    
    /// 获取可用显存
    pub fn available_vram(&self) -> u64 {
        self.gpu_status.read().iter().map(|s| s.free_memory_mb).sum()
    }
}
