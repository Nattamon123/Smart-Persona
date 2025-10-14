use axum::{routing::post, Router};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    domain::usecase::ai_analysis::AIAnalysisUseCase,
    infrastructure::ai_service_client::client::AIServiceClient,
};

pub fn routes(ai_use_case: Arc<AIAnalysisUseCase<AIServiceClient>>) -> Router {
    Router::new()
        .route("/analyze-personality", post(analyze_personality_handler).with_state(Arc::clone(&ai_use_case)))
        .route("/chat", post(chat_handler).with_state(ai_use_case))
}

#[derive(Deserialize)]
pub struct AnalyzePersonalityPayload {
    pub user_id: String,
    pub posts: Vec<String>,
}

#[derive(Serialize)]
pub struct AnalyzePersonalityResponse {
    pub personality_tags: Vec<String>,
    pub suggested_theme: String,
}

#[derive(Deserialize)]
pub struct ChatPayload {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatHandlerResponse {
    pub reply: String,
}

pub async fn analyze_personality_handler(
    State(ai_use_case): State<Arc<AIAnalysisUseCase<AIServiceClient>>>,
    Json(payload): Json<AnalyzePersonalityPayload>,
) -> impl IntoResponse {
    match ai_use_case.analyze_user_personality(payload.user_id, payload.posts).await {
        Ok(result) => {
            let response = AnalyzePersonalityResponse {
                personality_tags: result.personality_tags,
                suggested_theme: result.suggested_theme,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn chat_handler(
    State(ai_use_case): State<Arc<AIAnalysisUseCase<AIServiceClient>>>,
    Json(payload): Json<ChatPayload>,
) -> impl IntoResponse {
    match ai_use_case.chat_with_bot(payload.message).await {
        Ok(result) => {
            let response = ChatHandlerResponse {
                reply: result.reply,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
