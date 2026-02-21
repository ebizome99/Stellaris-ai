//! 配置管理模块
//! 
//! 支持从文件加载配置，提供默认配置

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::core::error::Result;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用版本
    pub version: String,
    /// 存储路径
    pub storage_path: PathBuf,
    /// 缓存配置
    pub cache: CacheConfig,
    /// GPU配置
    pub gpu: GPUConfig,
    /// 调度器配置
    pub scheduler: SchedulerConfig,
    /// 云端配置
    pub cloud: CloudConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// UI配置
    pub ui: UIConfig,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 启用缓存
    pub enabled: bool,
    /// 最大缓存大小 (MB)
    pub max_size_mb: u64,
    /// LRU缓存条目数
    pub lru_capacity: usize,
    /// 缓存过期时间 (秒)
    pub ttl_seconds: u64,
}

/// GPU配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUConfig {
    /// 启用GPU加速
    pub enabled: bool,
    /// 低显存模式
    pub low_vram_mode: bool,
    /// 显存限制 (MB), 0表示自动检测
    pub vram_limit_mb: u64,
    /// FP16精度
    pub fp16: bool,
    /// 自动混合精度
    pub auto_mixed_precision: bool,
    /// 模型Offload到CPU
    pub cpu_offload: bool,
    /// 批量大小
    pub default_batch_size: usize,
    /// 最大分辨率
    pub max_resolution: (u32, u32),
    /// 调度策略
    pub scheduling_strategy: SchedulingStrategy,
}

/// GPU调度策略
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SchedulingStrategy {
    /// 轮询
    #[default]
    RoundRobin,
    /// 最小负载
    LeastLoad,
    /// 显存优先
    VRAMFirst,
    /// 手动绑定
    Manual,
}

/// 调度器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// 最大并发任务
    pub max_concurrent_tasks: usize,
    /// 任务队列容量
    pub queue_capacity: usize,
    /// 任务超时 (秒)
    pub task_timeout_seconds: u64,
    /// 优先级队列数量
    pub priority_levels: usize,
    /// 自动降级阈值 (显存使用率%)
    pub auto_fallback_threshold: f32,
    /// 云端切换阈值 (本地负载%)
    pub cloud_switch_threshold: f32,
}

/// 云端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    /// 启用云端
    pub enabled: bool,
    /// 提供商列表
    pub providers: Vec<CloudProviderConfig>,
    /// 自动轮换
    pub auto_rotate: bool,
    /// 负载均衡
    pub load_balance: bool,
    /// 每日限额 (美元)
    pub daily_limit_usd: f64,
    /// 每月限额 (美元)
    pub monthly_limit_usd: f64,
}

/// 云端提供商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProviderConfig {
    /// 提供商名称
    pub name: String,
    /// 提供商类型
    pub provider_type: CloudProviderType,
    /// API端点
    pub endpoint: String,
    /// API密钥 (加密存储)
    pub encrypted_api_key: Option<String>,
    /// 是否启用
    pub enabled: bool,
    /// 优先级
    pub priority: u8,
    /// 每日限额 (美元)
    pub daily_limit: Option<f64>,
    /// 速率限制 (请求/分钟)
    pub rate_limit: Option<u32>,
}

/// 云端提供商类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloudProviderType {
    OpenAI,
    Google,
    StabilityAI,
    Replicate,
    Custom,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// API密钥加密
    pub encrypt_api_keys: bool,
    /// 日志脱敏
    pub sanitize_logs: bool,
    /// 插件沙箱
    pub plugin_sandbox: bool,
    /// 进程隔离
    pub process_isolation: bool,
}

/// UI配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// 语言
    pub language: String,
    /// 主题
    pub theme: Theme,
    /// 跟随系统主题
    pub follow_system_theme: bool,
    /// 动画效果
    pub animations: bool,
    /// 通知
    pub notifications: bool,
}

/// 主题
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let storage_path = home.join(".stellaris-ai");
        
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            storage_path,
            cache: CacheConfig {
                enabled: true,
                max_size_mb: 2048,
                lru_capacity: 100,
                ttl_seconds: 86400,
            },
            gpu: GPUConfig {
                enabled: true,
                low_vram_mode: false,
                vram_limit_mb: 0,
                fp16: true,
                auto_mixed_precision: true,
                cpu_offload: false,
                default_batch_size: 1,
                max_resolution: (2048, 2048),
                scheduling_strategy: SchedulingStrategy::default(),
            },
            scheduler: SchedulerConfig {
                max_concurrent_tasks: 4,
                queue_capacity: 100,
                task_timeout_seconds: 300,
                priority_levels: 3,
                auto_fallback_threshold: 0.9,
                cloud_switch_threshold: 0.8,
            },
            cloud: CloudConfig {
                enabled: true,
                providers: vec![],
                auto_rotate: true,
                load_balance: true,
                daily_limit_usd: 50.0,
                monthly_limit_usd: 1000.0,
            },
            security: SecurityConfig {
                encrypt_api_keys: true,
                sanitize_logs: true,
                plugin_sandbox: true,
                process_isolation: true,
            },
            ui: UIConfig {
                language: "zh-CN".to_string(),
                theme: Theme::default(),
                follow_system_theme: true,
                animations: true,
                notifications: true,
            },
        }
    }
}

impl AppConfig {
    /// 从文件加载配置
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let content = std::fs::read_to_string(&config_path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// 保存配置到文件
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
    
    /// 获取配置文件路径
    fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| crate::core::error::Error::Config("Cannot determine home directory".into()))?;
        Ok(home.join(".stellaris-ai").join("config.toml"))
    }
    
    /// 获取模型存储路径
    pub fn models_path(&self) -> PathBuf {
        self.storage_path.join("models")
    }
    
    /// 获取缓存路径
    pub fn cache_path(&self) -> PathBuf {
        self.storage_path.join("cache")
    }
    
    /// 获取输出路径
    pub fn output_path(&self) -> PathBuf {
        self.storage_path.join("output")
    }
    
    /// 获取日志路径
    pub fn logs_path(&self) -> PathBuf {
        self.storage_path.join("logs")
    }
}
