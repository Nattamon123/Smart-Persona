use async_trait::async_trait;
use anyhow::{Result, Context};
use reqwest::Client;

use crate::domain::{
    entities::ai_analysis::{AIAnalysisRequest, AIAnalysisResponse, ChatRequest, ChatResponse},
    repo::ai_service::AIServiceRepository,
};



pub struct AIServiceClient {
    client: Client,
    base_url: String,
}

impl AIServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl AIServiceRepository for AIServiceClient {
    async fn analyze_personality(&self, request: AIAnalysisRequest) -> Result<AIAnalysisResponse> {
        let url = format!("{}/analyze-personality", self.base_url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to AI service")?;

        if response.status().is_success() {
            let result = response
                .json::<AIAnalysisResponse>()
                .await
                .context("Failed to deserialize AI service success response")?;
            Ok(result)
        } else {
            let error_body = response.text().await.context("Failed to read AI service error body")?;
            Err(anyhow::anyhow!("AI service returned an error: {}", error_body))
        }
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/chat", self.base_url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send chat request to AI service")?;

        if response.status().is_success() {
            let result = response
                .json::<ChatResponse>()
                .await
                .context("Failed to deserialize chat service success response")?;
            Ok(result)
        } else {
            let error_body = response.text().await.context("Failed to read chat service error body")?;
            Err(anyhow::anyhow!("Chat service returned an error: {}", error_body))
        }
    }
}
