//! 模型注册表

use std::collections::HashMap;
use parking_lot::RwLock;
use crate::core::types::{ModelInfo, ModelType, GPUId};
use crate::core::error::Result;

/// 模型注册表
pub struct ModelRegistry {
    models: RwLock<HashMap<String, ModelInfo>>,
}

impl ModelRegistry {
    /// 创建新的模型注册表
    pub fn new() -> Self {
        Self {
            models: RwLock::new(HashMap::new()),
        }
    }
    
    /// 注册模型
    pub fn register(&self, model: ModelInfo) {
        self.models.write().insert(model.id.clone(), model);
    }
    
    /// 注销模型
    pub fn unregister(&self, id: &str) -> Option<ModelInfo> {
        self.models.write().remove(id)
    }
    
    /// 获取模型
    pub fn get(&self, id: &str) -> Option<ModelInfo> {
        self.models.read().get(id).cloned()
    }
    
    /// 获取所有模型
    pub fn get_all(&self) -> Vec<ModelInfo> {
        self.models.read().values().cloned().collect()
    }
    
    /// 获取已加载的模型
    pub fn get_loaded(&self) -> Vec<ModelInfo> {
        self.models
            .read()
            .values()
            .filter(|m| m.loaded)
            .cloned()
            .collect()
    }
    
    /// 更新模型加载状态
    pub fn set_loaded(&self, id: &str, loaded: bool, gpu_id: Option<GPUId>) {
        if let Some(model) = self.models.write().get_mut(id) {
            model.loaded = loaded;
            model.loaded_on_gpu = gpu_id;
        }
    }
    
    /// 获取模型数量
    pub fn count(&self) -> usize {
        self.models.read().len()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}
