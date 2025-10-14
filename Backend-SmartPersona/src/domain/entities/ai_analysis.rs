use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AIAnalysisRequest {
    pub user_id: String,
    pub posts: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AIAnalysisResponse {
    pub personality_tags: Vec<String>,
    pub suggested_theme: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub reply: String,
}
