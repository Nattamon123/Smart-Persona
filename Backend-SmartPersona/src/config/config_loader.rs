use anyhow::Result;
use super::{
    config_model::{Config, Application, Server, Database, Jwt, JwtSecret, JwtAdminSecret, Services},
    stage::Stage,
};

pub fn load() -> Result<Config> {
    dotenvy::dotenv().ok();
    
    // Load Application config
    let app = Application {
        env: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
        name: std::env::var("APP_NAME").expect("APP_NAME not set"),
        frontend_url: std::env::var("FRONTEND_URL").expect("FRONTEND_URL not set"),
        backend_url: std::env::var("BACKEND_URL").expect("BACKEND_URL not set"),
    };

    // Load Server config
    let server = Server {
        port: std::env::var("SERVER_PORT").expect("port not set").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("body_limit not set").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("timeout not set").parse()?,
    };

    // Load Database config
    let database = Database {
        url: std::env::var("DATABASE_URL").expect("database_url not set"),
    };

    // Load JWT config
    let jwt = Jwt {
        user: JwtSecret {
            user_secret: std::env::var("JWT_USER_SECRET").expect("JWT_USER_SECRET not set"),
            user_refresh_secret: std::env::var("JWT_USER_REFRESH_SECRET").expect("JWT_USER_REFRESH_SECRET not set"),
        },
        admin: JwtAdminSecret {
            admin_secret: std::env::var("JWT_ADMIN_SECRET").expect("JWT_ADMIN_SECRET not set"),
            admin_refresh_secret: std::env::var("JWT_ADMIN_REFRESH_SECRET").expect("JWT_ADMIN_REFRESH_SECRET not set"),
        },
        access_token_expiration: std::env::var("JWT_ACCESS_TOKEN_EXPIRATION").expect("JWT_ACCESS_TOKEN_EXPIRATION not set"),
        refresh_token_expiration: std::env::var("JWT_REFRESH_TOKEN_EXPIRATION").expect("JWT_REFRESH_TOKEN_EXPIRATION not set"),
    };

    // Load Services config
    let services = Services {
        ai_service_url: std::env::var("AI_SERVICE_URL").expect("AI_SERVICE_URL not set"),
    };

  

    Ok(Config { app, server, database, jwt, services })
}

pub fn get_stage() -> Stage{
    dotenvy::dotenv().ok();
    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}
pub fn get_user_secret() -> Result<JwtSecret> {
    dotenvy::dotenv().ok();
    Ok(JwtSecret {
        user_secret: std::env::var("JWT_USER_SECRET").expect("JWT_USER_SECRET not set"),
        user_refresh_secret: std::env::var("JWT_USER_REFRESH_SECRET")
            .expect("JWT_USER_REFRESH_SECRET not set"),
    })
}
pub fn get_admin_secret() -> Result<JwtAdminSecret> {
    dotenvy::dotenv().ok();
    Ok(JwtAdminSecret {
        admin_secret: std::env::var("JWT_ADMIN_SECRET")
            .expect("JWT_ADMIN_SECRET not set"),
        admin_refresh_secret: std::env::var("JWT_ADMIN_REFRESH_SECRET")
            .expect("JWT_ADMIN_REFRESH_SECRET not set"),
    })
}
