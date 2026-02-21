//! 混合算力调度器
//! 
//! 智能调度本地GPU和云端API

use std::sync::Arc;
use parking_lot::RwLock;
use crate::core::types::{GenerationParams, GenerationResult, TaskId, ExecutionMode};
use crate::core::error::{Error, Result};
use crate::core::config::AppConfig;
use crate::engine::local::LocalEngine;
use crate::engine::cloud::CloudEngine;
use crate::engine::provider::ModelProvider;

/// 混合算力调度器
pub struct HybridComputeScheduler {
    /// 应用配置
    config: Arc<RwLock<AppConfig>>,
    /// 本地引擎
    local_engine: Arc<LocalEngine>,
    /// 云端引擎
    cloud_engine: Arc<CloudEngine>,
    /// 统计信息
    stats: RwLock<SchedulerStats>,
}

/// 调度统计
#[derive(Debug, Clone, Default)]
pub struct SchedulerStats {
    /// 本地任务数
    pub local_tasks: u64,
    /// 云端任务数
    pub cloud_tasks: u64,
    /// 降级次数
    pub fallback_count: u64,
    /// 本地失败次数
    pub local_failures: u64,
    /// 云端失败次数
    pub cloud_failures: u64,
    /// 总节省成本 (美元)
    pub cost_saved_usd: f64,
}

impl HybridComputeScheduler {
    /// 创建新的混合调度器
    pub async fn new(
        config: Arc<RwLock<AppConfig>>,
        local_engine: Arc<LocalEngine>,
        cloud_engine: Arc<CloudEngine>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            local_engine,
            cloud_engine,
            stats: RwLock::new(SchedulerStats::default()),
        })
    }
    
    /// 生成图像
    pub async fn generate(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult> {
        let mode = self.decide_execution_mode(&params).await;
        
        match mode {
            ExecutionMode::Local => {
                self.generate_local(task_id, params).await
            }
            ExecutionMode::Cloud => {
                self.generate_cloud(task_id, params).await
            }
            ExecutionMode::Auto => {
                self.generate_auto(task_id, params).await
            }
        }
    }
    
    /// 决定执行模式
    async fn decide_execution_mode(&self, params: &GenerationParams) -> ExecutionMode {
        if params.execution_mode != ExecutionMode::Auto {
            return params.execution_mode;
        }
        
        let threshold = self.get_cloud_switch_threshold();
        
        let local_available = self.local_engine.is_available().await;
        let cloud_available = self.cloud_engine.is_available().await;
        
        if !local_available && !cloud_available {
            return ExecutionMode::Local;
        }
        
        if !local_available && cloud_available {
            return ExecutionMode::Cloud;
        }
        
        if !cloud_available && local_available {
            return ExecutionMode::Local;
        }
        
        let local_load = self.local_engine.get_load().await;
        
        if local_load > threshold {
            tracing::info!("本地负载过高 ({}%), 切换到云端", local_load * 100.0);
            return ExecutionMode::Cloud;
        }
        
        ExecutionMode::Local
    }
    
    /// 获取云端切换阈值
    fn get_cloud_switch_threshold(&self) -> f32 {
        let config = self.config.read();
        config.scheduler.cloud_switch_threshold
    }
    
    /// 本地生成
    async fn generate_local(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult> {
        match self.local_engine.generate(task_id, params).await {
            Ok(result) => {
                self.stats.write().local_tasks += 1;
                Ok(result)
            }
            Err(e) => {
                self.stats.write().local_failures += 1;
                Err(e)
            }
        }
    }
    
    /// 云端生成
    async fn generate_cloud(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult> {
        match self.cloud_engine.generate(task_id, params).await {
            Ok(result) => {
                self.stats.write().cloud_tasks += 1;
                Ok(result)
            }
            Err(e) => {
                self.stats.write().cloud_failures += 1;
                Err(e)
            }
        }
    }
    
    /// 自动模式生成 (带回退)
    async fn generate_auto(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult> {
        let mode = self.decide_execution_mode(&params).await;
        
        match mode {
            ExecutionMode::Local => {
                match self.generate_local(task_id, params.clone()).await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        tracing::warn!("本地生成失败: {}, 尝试云端", e);
                        
                        if self.cloud_engine.is_available().await {
                            self.stats.write().fallback_count += 1;
                            self.generate_cloud(task_id, params).await
                        } else {
                            Err(e)
                        }
                    }
                }
            }
            ExecutionMode::Cloud => {
                match self.generate_cloud(task_id, params.clone()).await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        tracing::warn!("云端生成失败: {}, 尝试本地", e);
                        
                        if self.local_engine.is_available().await {
                            self.stats.write().fallback_count += 1;
                            self.generate_local(task_id, params).await
                        } else {
                            Err(e)
                        }
                    }
                }
            }
            ExecutionMode::Auto => {
                self.generate_local(task_id, params).await
            }
        }
    }
    
    /// 取消任务
    pub async fn cancel(&self, task_id: TaskId) -> Result<()> {
        self.local_engine.cancel(task_id).await?;
        self.cloud_engine.cancel(task_id).await?;
        Ok(())
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> SchedulerStats {
        self.stats.read().clone()
    }
    
    /// 估算成本
    pub fn estimate_cost(&self, params: &GenerationParams) -> f64 {
        let base_cost = 0.02;
        let resolution_factor = (params.width * params.height) as f64 / (1024.0 * 1024.0);
        let batch_factor = params.batch_size as f64;
        
        base_cost * resolution_factor * batch_factor
    }
}
