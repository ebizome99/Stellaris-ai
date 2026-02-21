//! 应用常量定义

/// 应用信息
pub const APP_NAME: &str = "Stellaris AI";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 默认值
pub mod defaults {
    /// 默认生成宽度
    pub const DEFAULT_WIDTH: u32 = 1024;
    /// 默认生成高度
    pub const DEFAULT_HEIGHT: u32 = 1024;
    /// 默认步数
    pub const DEFAULT_STEPS: u32 = 20;
    /// 默认CFG Scale
    pub const DEFAULT_CFG_SCALE: f32 = 7.0;
    /// 默认采样器
    pub const DEFAULT_SAMPLER: &str = "euler_a";
    /// 默认批量大小
    pub const DEFAULT_BATCH_SIZE: u32 = 1;
}

/// 限制值
pub mod limits {
    /// 最小分辨率
    pub const MIN_RESOLUTION: u32 = 256;
    /// 最大分辨率
    pub const MAX_RESOLUTION: u32 = 4096;
    /// 最小步数
    pub const MIN_STEPS: u32 = 1;
    /// 最大步数
    pub const MAX_STEPS: u32 = 150;
    /// 最小CFG Scale
    pub const MIN_CFG_SCALE: f32 = 1.0;
    /// 最大CFG Scale
    pub const MAX_CFG_SCALE: f32 = 30.0;
    /// 最小批量大小
    pub const MIN_BATCH_SIZE: u32 = 1;
    /// 最大批量大小
    pub const MAX_BATCH_SIZE: u32 = 16;
    /// 低显存阈值 (MB)
    pub const LOW_VRAM_THRESHOLD_MB: u64 = 8192;
    /// 极低显存阈值 (MB)
    pub const VERY_LOW_VRAM_THRESHOLD_MB: u64 = 4096;
}

/// 支持的采样器
pub mod samplers {
    pub const EULER_A: &str = "euler_a";
    pub const EULER: &str = "euler";
    pub const HEUN: &str = "heun";
    pub const DPM_2: &str = "dpm_2";
    pub const DPM_2_A: &str = "dpm_2_a";
    pub const DPM_PP_2M: &str = "dpm_pp_2m";
    pub const DPM_PP_2M_SDE: &str = "dpm_pp_2m_sde";
    pub const DPM_PP_SDE: &str = "dpm_pp_sde";
    pub const DDIM: &str = "ddim";
    pub const UNI_PC: &str = "uni_pc";
    
    pub const ALL: &[&str] = &[
        EULER_A, EULER, HEUN, DPM_2, DPM_2_A,
        DPM_PP_2M, DPM_PP_2M_SDE, DPM_PP_SDE,
        DDIM, UNI_PC
    ];
}

/// 文件扩展名
pub mod extensions {
    pub const MODEL_SAFETENSORS: &str = "safetensors";
    pub const MODEL_CKPT: &str = "ckpt";
    pub const MODEL_BIN: &str = "bin";
    pub const IMAGE_PNG: &str = "png";
    pub const IMAGE_JPEG: &str = "jpg";
    pub const IMAGE_WEBP: &str = "webp";
}

/// 超时设置 (毫秒)
pub mod timeouts {
    /// 任务超时
    pub const TASK_TIMEOUT_MS: u64 = 300_000;
    /// API请求超时
    pub const API_REQUEST_TIMEOUT_MS: u64 = 30_000;
    /// 流式响应超时
    pub const STREAM_TIMEOUT_MS: u64 = 60_000;
    /// 健康检查间隔
    pub const HEALTH_CHECK_INTERVAL_MS: u64 = 5_000;
}
