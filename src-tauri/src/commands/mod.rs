//! Tauri命令模块
//! 
//! 前后端通信接口

use tauri::State;
use crate::AppState;
use crate::core::types::{GenerationParams, GenerationResult, TaskId, TaskStatus, TaskInfo};
use crate::core::error::Result;

/// 生成图像
#[tauri::command]
pub async fn generate_image(
    state: State<'_, crate::AppState>,
    prompt: String,
    negative_prompt: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    steps: Option<u32>,
    cfg_scale: Option<f32>,
    seed: Option<u64>,
    batch_size: Option<u32>,
    model: Option<String>,
) -> Result<GenerationResult> {
    let params = GenerationParams {
        prompt,
        negative_prompt,
        width: width.unwrap_or(1024),
        height: height.unwrap_or(1024),
        steps: steps.unwrap_or(20),
        cfg_scale: cfg_scale.unwrap_or(7.0),
        seed,
        batch_size: batch_size.unwrap_or(1),
        sampler: "euler_a".to_string(),
        model: model.unwrap_or_else(|| "sdxl".to_string()),
        execution_mode: crate::core::types::ExecutionMode::Auto,
        priority: crate::core::types::Priority::Normal,
    };
    
    let task_id = TaskId::new();
    
    state
        .hybrid_scheduler
        .generate(task_id, params)
        .await
}

/// 获取GPU状态
#[tauri::command]
pub fn get_gpu_status(
    state: State<'_, crate::AppState>,
) -> Vec<crate::core::types::GPUStatus> {
    state.gpu_scheduler.get_all_gpu_status()
}

/// 获取任务状态
#[tauri::command]
pub fn get_task_status(
    state: State<'_, crate::AppState>,
    task_id: TaskId,
) -> Option<TaskInfo> {
    None
}

/// 取消任务
#[tauri::command]
pub async fn cancel_task(
    state: State<'_, crate::AppState>,
    task_id: TaskId,
) -> Result<()> {
    state.hybrid_scheduler.cancel(task_id).await
}

/// 获取模型列表
#[tauri::command]
pub fn get_models(
    state: State<'_, crate::AppState>,
) -> Vec<crate::core::types::ModelInfo> {
    vec![]
}

/// 加载模型
#[tauri::command]
pub async fn load_model(
    state: State<'_, crate::AppState>,
    model_id: String,
) -> Result<()> {
    Ok(())
}

/// 卸载模型
#[tauri::command]
pub async fn unload_model(
    state: State<'_, crate::AppState>,
    model_id: String,
) -> Result<()> {
    Ok(())
}

/// 获取配置
#[tauri::command]
pub fn get_config(
    state: State<'_, crate::AppState>,
) -> crate::core::config::AppConfig {
    state.config.read().clone()
}

/// 更新配置
#[tauri::command]
pub fn update_config(
    state: State<'_, crate::AppState>,
    config: crate::core::config::AppConfig,
) -> Result<()> {
    let mut current = state.config.write();
    *current = config;
    current.save()?;
    Ok(())
}
