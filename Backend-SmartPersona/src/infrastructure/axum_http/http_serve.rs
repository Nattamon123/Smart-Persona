use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{http::{self, Method}, routing::{get, post}, Router};
use tokio::net::TcpListener;
use tower_http::{cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::{ TraceLayer}};
use tracing::info;

use crate::{
    config::config_model::Config as DotEnvyConfig,
    domain::usecase::ai_analysis::AIAnalysisUseCase,
    infrastructure::{
        ai_service_client::client::AIServiceClient,
        axum_http::{ default_routers, routers::{self, ai_handlers}},
        postgres::postgres_connection::DbPool,
    },
};

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<DbPool>) -> Result<()> {
    let ai_service_client = Arc::new(AIServiceClient::new("http://localhost:8001".to_string()));
    let ai_analysis_use_case = Arc::new(AIAnalysisUseCase::new(ai_service_client));

    let app = Router::new()
        .fallback(default_routers::not_found)
        .nest("/users", routers::user::routes(Arc::clone(&db_pool)))
        .nest("/authentication", routers::authentication::routes(Arc::clone(&db_pool)))
        .route("/health-check", get(default_routers::health_check))
        .nest("/api/ai", ai_handlers::routes(ai_analysis_use_case))
        .layer(TimeoutLayer::new(Duration::from_secs(config.server.timeout)))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
                .allow_credentials(true)
                .allow_headers([axum::http::header::AUTHORIZATION,axum::http::header::CONTENT_TYPE])
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;
    info!("Server running on {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    };
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => info!("Ctrl+C received, shutting down"),
        _ = terminate => info!("Terminate signal received, shutting down"),
    };
}