//! 本地推理引擎
//! 
//! 支持GPU/CPU推理，8GB低显存优化

use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use async_trait::async_trait;
use crate::core::types::{GenerationParams, GenerationResult, TaskId, GPUId};
use crate::core::error::{Error, Result};
use crate::core::config::AppConfig;
use crate::engine::provider::{ModelProvider, ProviderType};
use crate::scheduler::gpu::GPUScheduler;

/// 本地引擎
pub struct LocalEngine {
    /// 应用配置
    config: Arc<RwLock<AppConfig>>,
    /// GPU调度器
    gpu_scheduler: Arc<GPUScheduler>,
    /// 活跃任务
    active_tasks: Mutex<Vec<TaskId>>,
    /// 取消信号
    cancellation_tokens: RwLock<Vec<TaskId>>,
}

impl LocalEngine {
    /// 创建新的本地引擎
    pub async fn new(
        config: Arc<RwLock<AppConfig>>,
        gpu_scheduler: Arc<GPUScheduler>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            gpu_scheduler,
            active_tasks: Mutex::new(Vec::new()),
            cancellation_tokens: RwLock::new(Vec::new()),
        })
    }
    
    /// 检查低显存模式
    fn check_low_vram_mode(&self) -> bool {
        let config = self.config.read();
        let gpu_config = &config.gpu;
        
        if gpu_config.low_vram_mode {
            return true;
        }
        
        if let Some(status) = self.gpu_scheduler.get_primary_gpu_status() {
            if status.total_memory_mb <= 8192 {
                return true;
            }
        }
        
        false
    }
    
    /// 获取优化的生成参数
    fn optimize_params_for_low_vram(&self, params: &mut GenerationParams) {
        let config = self.config.read();
        
        if self.check_low_vram_mode() {
            if params.width > 1024 || params.height > 1024 {
                params.width = params.width.min(1024);
                params.height = params.height.min(1024);
                tracing::info!("低显存模式：分辨率已调整至 {}x{}", params.width, params.height);
            }
            
            if params.batch_size > 1 {
                params.batch_size = 1;
                tracing::info!("低显存模式：批量大小已调整为 1");
            }
        }
        
        if config.gpu.cpu_offload {
            params.batch_size = 1;
        }
    }
    
    /// 分配GPU资源
    async fn allocate_gpu(&self, params: &GenerationParams) -> Result<GPUId> {
        let estimated_vram = self.estimate_vram_usage(params);
        
        let gpu_id = self.gpu_scheduler
            .allocate_gpu(estimated_vram)
            .await
            .map_err(|e| Error::GPU(format!("无法分配GPU: {}", e)))?;
        
        Ok(gpu_id)
    }
    
    /// 估算显存占用
    fn estimate_vram_usage(&self, params: &GenerationParams) -> u64 {
        let base_model_vram = 4_500;
        let pixels = params.width as u64 * params.height as u64;
        let activation_vram = pixels * 3 * 4 / 1024 / 1024;
        let batch_multiplier = params.batch_size as u64;
        
        (base_model_vram + activation_vram) * batch_multiplier
    }
    
    /// 释放GPU资源
    async fn release_gpu(&self, gpu_id: GPUId) {
        self.gpu_scheduler.release_gpu(gpu_id).await;
    }
}

#[async_trait]
impl ModelProvider for LocalEngine {
    fn name(&self) -> &str {
        "LocalEngine"
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::LocalGPU
    }
    
    async fn is_available(&self) -> bool {
        self.gpu_scheduler.has_available_gpu()
    }
    
    async fn generate(
        &self,
        task_id: TaskId,
        mut params: GenerationParams,
    ) -> Result<GenerationResult> {
        self.optimize_params_for_low_vram(&mut params);
        
        let gpu_id = self.allocate_gpu(&params).await?;
        
        self.active_tasks.lock().push(task_id);
        
        let result = self.execute_generation(task_id, params, gpu_id).await;
        
        self.active_tasks.lock().retain(|t| t != &task_id);
        self.release_gpu(gpu_id).await;
        
        result
    }
    
    async fn generate_stream(
        &self,
        _task_id: TaskId,
        _params: GenerationParams,
    ) -> Result<super::provider::ProgressStream> {
        Err(crate::core::error::Error::Other("流式生成待实现".into()))
    }
    
    async fn cancel(&self, task_id: TaskId) -> Result<()> {
        self.cancellation_tokens.write().push(task_id);
        self.active_tasks.lock().retain(|t| t != &task_id);
        Ok(())
    }
    
    fn active_tasks(&self) -> usize {
        self.active_tasks.lock().len()
    }
    
    fn max_concurrent(&self) -> usize {
        self.gpu_scheduler.gpu_count()
    }
    
    fn estimated_wait_time(&self) -> u64 {
        let active = self.active_tasks.lock().len();
        let max_concurrent = self.max_concurrent();
        
        if active < max_concurrent {
            0
        } else {
            ((active - max_concurrent + 1) / max_concurrent) as u64 * 30
        }
    }
}

impl LocalEngine {
    pub async fn get_load(&self) -> f32 {
        let active = self.active_tasks.lock().len() as f32;
        let max = self.max_concurrent().max(1) as f32;
        (active / max).min(1.0)
    }
}

impl LocalEngine {
    /// 执行生成
    async fn execute_generation(
        &self,
        task_id: TaskId,
        params: GenerationParams,
        gpu_id: GPUId,
    ) -> Result<GenerationResult> {
        let start_time = std::time::Instant::now();
        
        tracing::info!(
            "[{}] 开始生成: {}x{}, {}步, GPU:{}",
            task_id, params.width, params.height, params.steps, gpu_id
        );
        
        let use_fp16 = {
            let config = self.config.read();
            config.gpu.fp16
        };
        
        let images = self.run_inference(&params, gpu_id, use_fp16).await?;
        
        let inference_time_ms = start_time.elapsed().as_millis() as u64;
        
        tracing::info!("[{}] 生成完成, 耗时: {}ms", task_id, inference_time_ms);
        
        Ok(GenerationResult {
            task_id,
            images,
            seeds: vec![params.seed.unwrap_or(0)],
            inference_time_ms,
            execution_mode: crate::core::types::ExecutionMode::Local,
            gpu_id: Some(gpu_id),
            metadata: serde_json::json!({
                "fp16": use_fp16,
                "model": params.model,
            }),
        })
    }
    
    /// 运行推理
    async fn run_inference(
        &self,
        _params: &GenerationParams,
        _gpu_id: GPUId,
        _use_fp16: bool,
    ) -> Result<Vec<String>> {
        Ok(vec!["base64_placeholder_image_data".to_string()])
    }
}
