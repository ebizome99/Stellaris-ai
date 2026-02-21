//! 模型加载器

use crate::core::error::Result;
use crate::core::types::{ModelInfo, ModelType, GPUId};

/// 模型加载器
pub struct ModelLoader {
    models_dir: std::path::PathBuf,
}

impl ModelLoader {
    /// 创建新的模型加载器
    pub fn new(models_dir: std::path::PathBuf) -> Self {
        Self { models_dir }
    }
    
    /// 扫描可用模型
    pub fn scan_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models = Vec::new();
        
        if !self.models_dir.exists() {
            return Ok(models);
        }
        
        for entry in std::fs::read_dir(&self.models_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(ext) = path.extension() {
                if ext == "safetensors" || ext == "ckpt" {
                    if let Some(name) = path.file_stem() {
                        let name = name.to_string_lossy().to_string();
                        let metadata = std::fs::metadata(&path)?;
                        
                        models.push(ModelInfo {
                            id: name.clone(),
                            name,
                            model_type: ModelType::SDXL,
                            path: path.to_string_lossy().to_string(),
                            size_mb: metadata.len() / 1024 / 1024,
                            loaded: false,
                            loaded_on_gpu: None,
                            vram_usage_mb: None,
                            supported_resolutions: vec![
                                (1024, 1024),
                                (1024, 1792),
                                (1792, 1024),
                            ],
                            metadata: serde_json::json!({}),
                        });
                    }
                }
            }
        }
        
        Ok(models)
    }
    
    /// 加载模型到GPU
    pub async fn load_model(&self, model: &ModelInfo, gpu_id: GPUId) -> Result<()> {
        tracing::info!("加载模型 {} 到 {}", model.id, gpu_id);
        Ok(())
    }
    
    /// 卸载模型
    pub async fn unload_model(&self, model: &ModelInfo) -> Result<()> {
        tracing::info!("卸载模型 {}", model.id);
        Ok(())
    }
}
