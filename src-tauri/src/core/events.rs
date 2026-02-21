//! 事件定义模块
//! 
//! Tauri事件用于前后端通信

use serde::{Deserialize, Serialize};
use crate::core::types::{TaskId, TaskStatus, GPUId, GenerationResult};

/// 事件名称常量
pub mod event_names {
    /// 任务状态变更
    pub const TASK_STATUS_CHANGED: &str = "task:status-changed";
    /// 任务进度更新
    pub const TASK_PROGRESS: &str = "task:progress";
    /// 任务完成
    pub const TASK_COMPLETED: &str = "task:completed";
    /// 任务失败
    pub const TASK_FAILED: &str = "task:failed";
    /// GPU状态变更
    pub const GPU_STATUS_CHANGED: &str = "gpu:status-changed";
    /// 模型加载完成
    pub const MODEL_LOADED: &str = "model:loaded";
    /// 模型卸载完成
    pub const MODEL_UNLOADED: &str = "model:unloaded";
    /// 生成步骤更新
    pub const GENERATION_STEP: &str = "generation:step";
    /// 低显存警告
    pub const LOW_VRAM_WARNING: &str = "system:low-vram-warning";
    /// 云端切换
    pub const CLOUD_SWITCH: &str = "cloud:switch";
}

/// 任务状态变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusChangedEvent {
    /// 任务ID
    pub task_id: TaskId,
    /// 旧状态
    pub old_status: TaskStatus,
    /// 新状态
    pub new_status: TaskStatus,
    /// 原因
    pub reason: Option<String>,
}

/// 任务进度事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgressEvent {
    /// 任务ID
    pub task_id: TaskId,
    /// 进度 (0-100)
    pub progress: u8,
    /// 当前步骤
    pub step: u32,
    /// 总步数
    pub total_steps: u32,
    /// 步骤描述
    pub description: String,
}

/// 任务完成事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletedEvent {
    /// 任务ID
    pub task_id: TaskId,
    /// 结果
    pub result: GenerationResult,
}

/// 任务失败事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFailedEvent {
    /// 任务ID
    pub task_id: TaskId,
    /// 错误信息
    pub error: String,
    /// 是否会自动重试
    pub will_retry: bool,
}

/// GPU状态变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUStatusChangedEvent {
    /// GPU ID
    pub gpu_id: GPUId,
    /// 变更类型
    pub change_type: GPUChangeType,
}

/// GPU变更类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GPUChangeType {
    /// 任务开始
    TaskStarted,
    /// 任务结束
    TaskEnded,
    /// 显存变化
    MemoryChanged,
    /// 状态更新
    StatusUpdated,
}

/// 生成步骤事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStepEvent {
    /// 任务ID
    pub task_id: TaskId,
    /// 当前步骤
    pub step: String,
    /// 详情
    pub details: Option<String>,
}

/// 低显存警告事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowVRAMWarningEvent {
    /// 当前显存使用 (MB)
    pub used_mb: u64,
    /// 总显存 (MB)
    pub total_mb: u64,
    /// 使用率
    pub utilization: f32,
    /// 建议操作
    pub suggestions: Vec<String>,
}

/// 云端切换事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudSwitchEvent {
    /// 任务ID
    pub task_id: TaskId,
    /// 切换原因
    pub reason: String,
    /// 目标提供商
    pub provider: String,
}
