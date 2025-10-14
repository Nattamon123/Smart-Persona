use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{domain::{repo::user::UserRepository, usecase::user::UserUseCase, value_object::user::RegisterUserModel}, infrastructure::postgres::{postgres_connection::DbPool, repositories::user::UserPostgres}};



pub fn routes(db_pool: Arc<DbPool>) -> Router {
    let user_repository = UserPostgres::new(db_pool);
    let user_use_case = UserUseCase::new(Arc::new(user_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(user_use_case))
}

pub async fn register<T>(
    State(user_use_case): State<Arc<UserUseCase<T>>>,
    Json(register_user_model): Json<RegisterUserModel>,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    match user_use_case.register(register_user_model).await {
        Ok(user_id) => (
            StatusCode::CREATED,
            format!("Register user id: {} successfully", user_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
        
       
}