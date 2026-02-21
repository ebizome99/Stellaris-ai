//! OpenAI API提供商

use async_trait::async_trait;
use crate::core::types::{GenerationParams, GenerationResult, TaskId};
use crate::core::error::{Error, Result};
use crate::engine::provider::{ModelProvider, ProviderType, ProgressUpdate};

/// OpenAI提供商
pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl OpenAIProvider {
    /// 创建新的OpenAI提供商
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: None,
        }
    }
    
    /// 设置API密钥
    pub fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "OpenAI"
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::CloudOpenAI
    }
    
    async fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
    
    async fn generate(
        &self,
        task_id: TaskId,
        params: GenerationParams,
    ) -> Result<GenerationResult> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| Error::CloudAPI("API密钥未设置".into()))?;
        
        let size = match (params.width, params.height) {
            (1024, 1024) => "1024x1024",
            (1024, 1792) => "1024x1792",
            (1792, 1024) => "1792x1024",
            _ => "1024x1024",
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/images/generations")
            .bearer_auth(api_key)
            .json(&serde_json::json!({
                "model": "dall-e-3",
                "prompt": params.prompt,
                "n": params.batch_size.min(4) as i32,
                "size": size,
                "quality": "standard",
                "response_format": "b64_json"
            }))
            .send()
            .await
            .map_err(|e| Error::CloudAPI(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(Error::CloudAPI(format!("API错误: {}", error)));
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
        
        Ok(GenerationResult {
            task_id,
            images,
            seeds: vec![params.seed.unwrap_or(0)],
            inference_time_ms: 0,
            execution_mode: crate::core::types::ExecutionMode::Cloud,
            gpu_id: None,
            metadata: serde_json::json!({"provider": "openai", "model": "dall-e-3"}),
        })
    }
    
    async fn generate_stream(
        &self,
        _task_id: TaskId,
        _params: GenerationParams,
    ) -> Result<super::super::engine::provider::ProgressStream> {
        Err(Error::CloudAPI("OpenAI不支持流式生成".into()))
    }
    
    async fn cancel(&self, _task_id: TaskId) -> Result<()> {
        Ok(())
    }
    
    fn active_tasks(&self) -> usize {
        0
    }
    
    fn max_concurrent(&self) -> usize {
        5
    }
    
    fn estimated_wait_time(&self) -> u64 {
        10
    }
}
