use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::{post, get}, Json, Router};

use crate::{domain::usecase::authentication::AuthenticationUseCase, infrastructure::{jwt_authentication::authentication_model::LoginModel, postgres::postgres_connection::DbPool, axum_http::middleware::admin_authorization}};

pub fn routes(db_pool: Arc<DbPool>) -> Router {
    let user_repository = Arc::new(Arc::clone(&db_pool));
    let auth_usecase = Arc::new(AuthenticationUseCase::new(user_repository));

    Router::new()
        .route("/login", post(admin_login_handler).with_state(auth_usecase.clone()))
        // Add other admin routes here that need protection
        .route("/dashboard", get(admin_dashboard_handler))
        .route_layer(axum::middleware::from_fn(admin_authorization))
}

async fn admin_login_handler(
    State(auth_usecase): State<Arc<AuthenticationUseCase<UserPostgres>>>,
    Json(payload): Json<LoginModel>,
) -> Result<Json<crate::infrastructure::jwt_authentication::jwt_model::Passport>, StatusCode> {
    match auth_usecase.admin_login(payload).await {
        Ok(passport) => Ok(Json(passport)),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

// Example protected handler
async fn admin_dashboard_handler() -> &'static str {
    "Welcome to the Admin Dashboard!"
}
