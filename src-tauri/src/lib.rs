//! Stellaris AI - Enterprise AI Image Generation System
//! 
//! 企业级AI图片生成系统，支持本地+云端混合架构
//! 多GPU并行调度，8GB低显存优化

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::sync::Arc;
use parking_lot::RwLock;

pub mod core;
pub mod engine;
pub mod scheduler;
pub mod cloud;
pub mod security;
pub mod models;
pub mod utils;
pub mod commands;

use core::config::AppConfig;
use core::error::Result;
use scheduler::gpu::GPUScheduler;
use scheduler::hybrid::HybridComputeScheduler;
use engine::local::LocalEngine;
use engine::cloud::CloudEngine;

/// 应用状态
pub struct AppState {
    /// 应用配置
    pub config: Arc<RwLock<AppConfig>>,
    /// GPU调度器
    pub gpu_scheduler: Arc<GPUScheduler>,
    /// 混合算力调度器
    pub hybrid_scheduler: Arc<HybridComputeScheduler>,
    /// 本地引擎
    pub local_engine: Arc<LocalEngine>,
    /// 云端引擎
    pub cloud_engine: Arc<CloudEngine>,
}

impl AppState {
    /// 创建新的应用状态
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config = Arc::new(RwLock::new(config));
        
        let gpu_scheduler = Arc::new(GPUScheduler::new(config.clone()).await?);
        let local_engine = Arc::new(LocalEngine::new(config.clone(), gpu_scheduler.clone()).await?);
        let cloud_engine = Arc::new(CloudEngine::new(config.clone()).await?);
        
        let hybrid_scheduler = Arc::new(
            HybridComputeScheduler::new(
                config.clone(),
                local_engine.clone(),
                cloud_engine.clone(),
            ).await?
        );
        
        Ok(Self {
            config,
            gpu_scheduler,
            hybrid_scheduler,
            local_engine,
            cloud_engine,
        })
    }
}
