//! Stability AI API提供商

use async_trait::async_trait;
use crate::core::types::{GenerationParams, GenerationResult, TaskId};
use crate::core::error::{Error, Result};
use crate::engine::provider::{ModelProvider, ProviderType};

/// Stability AI提供商
pub struct StabilityProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl StabilityProvider {
    /// 创建新的Stability提供商
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
impl ModelProvider for StabilityProvider {
    fn name(&self) -> &str {
        "Stability AI"
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::CloudStabilityAI
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
        
        let response = self.client
            .post("https://api.stability.ai/v1/generation/stable-diffusion-xl-1024-v1-0/text-to-image")
            .bearer_auth(api_key)
            .json(&serde_json::json!({
                "text_prompts": [
                    {"text": params.prompt, "weight": 1.0},
                ],
                "cfg_scale": params.cfg_scale,
                "height": params.height,
                "width": params.width,
                "samples": params.batch_size,
                "steps": params.steps,
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
        
        let images = result["artifacts"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item["base64"].as_str().map(|s| s.to_string()))
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
            metadata: serde_json::json!({"provider": "stability", "model": "sdxl"}),
        })
    }
    
    async fn generate_stream(
        &self,
        _task_id: TaskId,
        _params: GenerationParams,
    ) -> Result<super::super::engine::provider::ProgressStream> {
        Err(Error::CloudAPI("Stability不支持流式生成".into()))
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
        15
    }
}
