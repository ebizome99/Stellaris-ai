//! 核心类型定义

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 任务ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl TaskId {
    /// 生成新的任务ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// GPU ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GPUId(pub usize);

impl std::fmt::Display for GPUId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GPU{}", self.0)
    }
}

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// 低优先级
    Low,
    /// 普通优先级
    #[default]
    Normal,
    /// 高优先级
    High,
    /// 实时优先级
    Realtime,
}

impl Priority {
    /// 获取优先级数值 (越大越优先)
    pub fn value(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::High => 2,
            Self::Realtime => 3,
        }
    }
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 等待中
    Pending,
    /// 队列中
    Queued,
    /// 运行中
    Running,
    /// 暂停
    Paused,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
    /// 超时
    Timeout,
}

/// 执行模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    /// 本地
    #[default]
    Local,
    /// 云端
    Cloud,
    /// 自动选择
    Auto,
}

/// 图像生成参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    /// 提示词
    pub prompt: String,
    /// 负面提示词
    pub negative_prompt: Option<String>,
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 步数
    pub steps: u32,
    /// CFG Scale
    pub cfg_scale: f32,
    /// 种子
    pub seed: Option<u64>,
    /// 批量大小
    pub batch_size: u32,
    /// 采样器
    pub sampler: String,
    /// 模型
    pub model: String,
    /// 执行模式
    pub execution_mode: ExecutionMode,
    /// 优先级
    pub priority: Priority,
}

/// 图像生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    /// 任务ID
    pub task_id: TaskId,
    /// 图像数据 (Base64)
    pub images: Vec<String>,
    /// 种子
    pub seeds: Vec<u64>,
    /// 推理时间 (毫秒)
    pub inference_time_ms: u64,
    /// 执行模式
    pub execution_mode: ExecutionMode,
    /// 使用的GPU
    pub gpu_id: Option<GPUId>,
    /// 元数据
    pub metadata: serde_json::Value,
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    /// 任务ID
    pub id: TaskId,
    /// 生成参数
    pub params: GenerationParams,
    /// 状态
    pub status: TaskStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 开始时间
    pub started_at: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 结果
    pub result: Option<GenerationResult>,
    /// 错误信息
    pub error: Option<String>,
    /// 进度 (0-100)
    pub progress: u8,
    /// 当前步骤描述
    pub current_step: String,
}

/// GPU状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUStatus {
    /// GPU ID
    pub id: GPUId,
    /// GPU名称
    pub name: String,
    /// 总显存 (MB)
    pub total_memory_mb: u64,
    /// 已用显存 (MB)
    pub used_memory_mb: u64,
    /// 空闲显存 (MB)
    pub free_memory_mb: u64,
    /// 显存使用率
    pub memory_utilization: f32,
    /// GPU利用率
    pub gpu_utilization: f32,
    /// 温度 (°C)
    pub temperature: Option<u32>,
    /// 当前任务数
    pub active_tasks: usize,
    /// 队列长度
    pub queue_length: usize,
    /// 是否可用
    pub available: bool,
}

/// 模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// 模型ID
    pub id: String,
    /// 模型名称
    pub name: String,
    /// 模型类型
    pub model_type: ModelType,
    /// 文件路径
    pub path: String,
    /// 文件大小 (MB)
    pub size_mb: u64,
    /// 是否已加载
    pub loaded: bool,
    /// 加载到的GPU
    pub loaded_on_gpu: Option<GPUId>,
    /// 显存占用 (MB)
    pub vram_usage_mb: Option<u64>,
    /// 支持的分辨率
    pub supported_resolutions: Vec<(u32, u32)>,
    /// 元数据
    pub metadata: serde_json::Value,
}

/// 模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    /// Stable Diffusion 1.x
    SD1x,
    /// Stable Diffusion 2.x
    SD2x,
    /// Stable Diffusion XL
    SDXL,
    /// SDXL Turbo
    SDXLTurbo,
    /// Stable Diffusion 3
    SD3,
    /// 自定义
    Custom,
}
