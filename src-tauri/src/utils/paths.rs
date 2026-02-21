//! 路径工具函数

use std::path::PathBuf;

/// 应用路径
pub struct AppPaths {
    /// 配置目录
    pub config_dir: PathBuf,
    /// 数据目录
    pub data_dir: PathBuf,
    /// 缓存目录
    pub cache_dir: PathBuf,
    /// 日志目录
    pub logs_dir: PathBuf,
    /// 模型目录
    pub models_dir: PathBuf,
    /// 输出目录
    pub output_dir: PathBuf,
}

/// 获取应用路径
pub fn get_app_paths() -> AppPaths {
    let home = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    
    let base = home.join(".stellaris-ai");
    
    AppPaths {
        config_dir: base.join("config"),
        data_dir: base.join("data"),
        cache_dir: base.join("cache"),
        logs_dir: base.join("logs"),
        models_dir: base.join("models"),
        output_dir: base.join("output"),
    }
}

impl AppPaths {
    /// 确保所有目录存在
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        std::fs::create_dir_all(&self.logs_dir)?;
        std::fs::create_dir_all(&self.models_dir)?;
        std::fs::create_dir_all(&self.output_dir)?;
        Ok(())
    }
}
