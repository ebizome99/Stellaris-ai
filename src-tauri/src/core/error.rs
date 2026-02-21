//! 错误处理模块
//! 
//! 统一的错误类型定义和处理

use thiserror::Error;
use serde::{Deserialize, Serialize};

/// 应用程序错误类型
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum Error {
    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),
    
    /// GPU错误
    #[error("GPU错误: {0}")]
    GPU(String),
    
    /// 模型错误
    #[error("模型错误: {0}")]
    Model(String),
    
    /// 推理错误
    #[error("推理错误: {0}")]
    Inference(String),
    
    /// 调度错误
    #[error("调度错误: {0}")]
    Scheduler(String),
    
    /// 云端API错误
    #[error("云端API错误: {0}")]
    CloudAPI(String),
    
    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),
    
    /// IO错误
    #[error("IO错误: {0}")]
    IO(String),
    
    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(String),
    
    /// TOML解析错误
    #[error("TOML解析错误: {0}")]
    TomlParse(String),
    
    /// TOML序列化错误
    #[error("TOML序列化错误: {0}")]
    TomlSerialize(String),
    
    /// 加密错误
    #[error("加密错误: {0}")]
    Crypto(String),
    
    /// 安全错误
    #[error("安全错误: {0}")]
    Security(String),
    
    /// 任务错误
    #[error("任务错误: {0}")]
    Task(String),
    
    /// 插件错误
    #[error("插件错误: {0}")]
    Plugin(String),
    
    /// 通用错误
    #[error("{0}")]
    Other(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlParse(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlSerialize(err.to_string())
    }
}
