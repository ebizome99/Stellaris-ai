//! 应用程序入口点
//! 
//! 初始化Tauri应用并启动前端服务

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use stellaris_ai_lib::{AppState, core::config::AppConfig};
use tauri::Manager;

#[tokio::main]
async fn main() {
    stellaris_ai_lib::utils::logging::init_logging();
    
    tracing::info!("Starting Stellaris AI...");
    
    let config = AppConfig::load().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {}, using defaults", e);
        AppConfig::default()
    });
    
    let state = match AppState::new(config).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            stellaris_ai_lib::commands::generate_image,
            stellaris_ai_lib::commands::get_gpu_status,
            stellaris_ai_lib::commands::get_task_status,
            stellaris_ai_lib::commands::cancel_task,
            stellaris_ai_lib::commands::get_models,
            stellaris_ai_lib::commands::load_model,
            stellaris_ai_lib::commands::unload_model,
            stellaris_ai_lib::commands::update_config,
            stellaris_ai_lib::commands::get_config,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            
            tracing::info!("Stellaris AI initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
