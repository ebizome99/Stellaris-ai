//! 模型提供者抽象接口
//! 
//! 统一的模型提供者Trait，支持本地和云端引擎

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use crate::core::types::{GenerationParams, GenerationResult, TaskId};
use crate::core::error::Result;

/// 生成进度流
pub type ProgressStream = Pin<Box<dyn Stream<Item = ProgressUpdate> + Send>>;

/// 进度更新
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    /// 任务ID
    pub task_id: TaskId,
    /// 进度 (0-100)
    pub progress: u8,
    /// 当前步骤
    pub step: u32,
    /// 总步数
    pub total_steps: u32,
    /// 描述
    pub description: String,
    /// 预览图像 (Base64, 可选)
    pub preview: Option<String>,
}

/// 模型提供者Trait
/// 
/// 所有模型引擎（本地/云端）必须实现此接口
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// 提供者名称
    fn name(&self) -> &str;
    
    /// 提供者类型
    fn provider_type(&self) -> ProviderType;
    
    /// 是否可用
    async fn is_available(&self) -> bool;
    
    /// 生成图像
    async fn generate(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult>;
    
    /// 流式生成（返回进度流）
    async fn generate_stream(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<ProgressStream>;
    
    /// 取消任务
    async fn cancel(&self, task_id: TaskId) -> Result<()>;
    
    /// 获取当前任务数
    fn active_tasks(&self) -> usize;
    
    /// 获取最大并发数
    fn max_concurrent(&self) -> usize;
    
    /// 获取预估等待时间 (秒)
    fn estimated_wait_time(&self) -> u64;
}

/// 提供者类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderType {
    /// 本地GPU
    LocalGPU,
    /// 本地CPU
    LocalCPU,
    /// 云端API (OpenAI)
    CloudOpenAI,
    /// 云端API (Google)
    CloudGoogle,
    /// 云端API (Stability AI)
    CloudStabilityAI,
    /// 云端API (Replicate)
    CloudReplicate,
    /// 私有云
    PrivateCloud,
    /// 自定义
    Custom,
}

/// 提供者能力
#[derive(Debug, Clone)]
pub struct ProviderCapabilities {
    /// 支持流式生成
    pub supports_streaming: bool,
    /// 支持中断
    pub supports_cancellation: bool,
    /// 支持批量生成
    pub supports_batch: bool,
    /// 最大批量大小
    pub max_batch_size: usize,
    /// 支持的最大分辨率
    pub max_resolution: (u32, u32),
    /// 支持的采样器
    pub supported_samplers: Vec<String>,
    /// 预估每张图成本 (美元, None表示免费)
    pub cost_per_image: Option<f64>,
}

/// 提供者状态
#[derive(Debug, Clone)]
pub struct ProviderStatus {
    /// 提供者名称
    pub name: String,
    /// 是否在线
    pub is_online: bool,
    /// 当前队列长度
    pub queue_length: usize,
    /// 平均响应时间 (毫秒)
    pub avg_response_time_ms: u64,
    /// 最近错误
    pub last_error: Option<String>,
    /// 今日已用额度 (美元)
    pub daily_usage: f64,
    /// 今日限额 (美元)
    pub daily_limit: Option<f64>,
}
