use async_trait::async_trait;
use anyhow::Result;

use crate::domain::entities::ai_analysis::{AIAnalysisRequest, AIAnalysisResponse, ChatRequest, ChatResponse};

#[async_trait]
pub trait AIServiceRepository {
    async fn analyze_personality(&self, request: AIAnalysisRequest) -> Result<AIAnalysisResponse>;
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
}
