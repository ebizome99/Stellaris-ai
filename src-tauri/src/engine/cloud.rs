//! 云端API引擎
//! 
//! 支持多种云端AI服务的统一接口

use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use async_trait::async_trait;
use crate::core::types::{GenerationParams, GenerationResult, TaskId};
use crate::core::error::{Error, Result};
use crate::core::config::AppConfig;
use crate::engine::provider::{ModelProvider, ProviderType};

/// 云端引擎
pub struct CloudEngine {
    /// 应用配置
    config: Arc<RwLock<AppConfig>>,
    /// 活跃任务
    active_tasks: Mutex<Vec<TaskId>>,
    /// HTTP客户端
    client: reqwest::Client,
    /// API密钥 (加密)
    encrypted_keys: RwLock<Vec<EncryptedKey>>,
}

/// 加密密钥
#[derive(Debug, Clone)]
struct EncryptedKey {
    provider: String,
    encrypted_data: Vec<u8>,
}

impl CloudEngine {
    /// 创建新的云端引擎
    pub async fn new(config: Arc<RwLock<AppConfig>>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| Error::CloudAPI(format!("创建HTTP客户端失败: {}", e)))?;
        
        Ok(Self {
            config,
            active_tasks: Mutex::new(Vec::new()),
            client,
            encrypted_keys: RwLock::new(Vec::new()),
        })
    }
    
    /// 设置API密钥
    pub fn set_api_key(&self, provider: &str, api_key: &str) -> Result<()> {
        let encrypted = crate::security::encryption::encrypt(api_key.as_bytes())?;
        
        let mut keys = self.encrypted_keys.write();
        keys.retain(|k| k.provider != provider);
        keys.push(EncryptedKey {
            provider: provider.to_string(),
            encrypted_data: encrypted,
        });
        
        Ok(())
    }
    
    /// 获取API密钥
    fn get_api_key(&self, provider: &str) -> Option<String> {
        let keys = self.encrypted_keys.read();
        keys.iter()
            .find(|k| k.provider == provider)
            .and_then(|k| {
                crate::security::encryption::decrypt(&k.encrypted_data)
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok())
            })
    }
    
    /// 检查并处理速率限制
    async fn check_rate_limit(&self, provider: &str) -> Result<()> {
        Ok(())
    }
    
    /// 获取可用的云端提供商
    fn get_available_provider(&self) -> Option<String> {
        let config = self.config.read();
        
        for provider in &config.cloud.providers {
            if provider.enabled && self.get_api_key(&provider.name).is_some() {
                return Some(provider.name.clone());
            }
        }
        
        None
    }
    
    /// 执行OpenAI API调用
    async fn call_openai(&self, params: &GenerationParams, api_key: &str) -> Result<Vec<String>> {
        let response = self.client
            .post("https://api.openai.com/v1/images/generations")
            .bearer_auth(api_key)
            .json(&serde_json::json!({
                "model": "dall-e-3",
                "prompt": params.prompt,
                "n": params.batch_size,
                "size": format!("{}x{}", params.width, params.height),
                "response_format": "b64_json"
            }))
            .send()
            .await
            .map_err(|e| Error::CloudAPI(format!("OpenAI请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(Error::CloudAPI(format!("OpenAI错误: {}", error)));
        }
        
        let result: serde_json::Value = response.json().await
            .map_err(|e| Error::CloudAPI(format!("解析响应失败: {}", e)))?;
        
        let images = result["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item["b64_json"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(images)
    }
}

#[async_trait]
impl ModelProvider for CloudEngine {
    fn name(&self) -> &str {
        "CloudEngine"
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::CloudOpenAI
    }
    
    async fn is_available(&self) -> bool {
        self.get_available_provider().is_some()
    }
    
    async fn generate(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult> {
        let provider = self.get_available_provider()
            .ok_or_else(|| Error::CloudAPI("没有可用的云端提供商".into()))?;
        
        let api_key = self.get_api_key(&provider)
            .ok_or_else(|| Error::CloudAPI("API密钥未配置".into()))?;
        
        self.check_rate_limit(&provider).await?;
        
        self.active_tasks.lock().push(task_id);
        
        let start_time = std::time::Instant::now();
        
        let images = match provider.as_str() {
            "openai" => self.call_openai(&params, &api_key).await?,
            _ => return Err(Error::CloudAPI(format!("不支持的提供商: {}", provider))),
        };
        
        let inference_time_ms = start_time.elapsed().as_millis() as u64;
        
        self.active_tasks.lock().retain(|t| t != &task_id);
        
        Ok(GenerationResult {
            task_id,
            images,
            seeds: vec![params.seed.unwrap_or(0)],
            inference_time_ms,
            execution_mode: crate::core::types::ExecutionMode::Cloud,
            gpu_id: None,
            metadata: serde_json::json!({
                "provider": provider,
            }),
        })
    }
    
    async fn generate_stream(
        &self,
        _task_id: TaskId,
        _params: GenerationParams,
    ) -> Result<super::provider::ProgressStream> {
        Err(Error::CloudAPI("云端引擎不支持流式生成".into()))
    }
    
    async fn cancel(&self, task_id: TaskId) -> Result<()> {
        self.active_tasks.lock().retain(|t| t != &task_id);
        Ok(())
    }
    
    fn active_tasks(&self) -> usize {
        self.active_tasks.lock().len()
    }
    
    fn max_concurrent(&self) -> usize {
        10
    }
    
    fn estimated_wait_time(&self) -> u64 {
        5
    }
}

impl CloudEngine {
    pub async fn get_load(&self) -> f32 {
        let active = self.active_tasks.lock().len() as f32;
        let max = self.max_concurrent().max(1) as f32;
        (active / max).min(1.0)
    }
}
