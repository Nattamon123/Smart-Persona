use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    config::config_loader::{get_admin_secret, get_user_secret},
    infrastructure::jwt_authentication::{self, jwt_model::{Claims, Roles}},
};
use uuid::Uuid;

pub async fn user_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(token) = get_cookie_value(cookie_str, "act") {
                if let Ok(user_secret) = get_user_secret() {
                    if let Ok(claims) = jwt_authentication::verify_token(user_secret.user_secret, token.clone()) {
                        if claims.role == Roles::UserAndCompany || claims.role == Roles::Admin {
                            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                                req.extensions_mut().insert(user_id);
                            }
                            req.extensions_mut().insert::<Claims>(claims);
                            return Ok(next.run(req).await);
                        }
                    }
                }
                if let Ok(admin_secret) = get_admin_secret() {
                    if let Ok(claims) = jwt_authentication::verify_token(admin_secret.admin_secret, token) {
                        if claims.role == Roles::Admin {
                            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                                req.extensions_mut().insert(user_id);
                            }
                            req.extensions_mut().insert::<Claims>(claims);
                            return Ok(next.run(req).await);
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn admin_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(token) = get_cookie_value(cookie_str, "act") {
                if let Ok(admin_secret) = get_admin_secret() {
                    if let Ok(claims) = jwt_authentication::verify_token(admin_secret.admin_secret, token) {
                        if claims.role == Roles::Admin {
                            if let Ok(admin_id) = Uuid::parse_str(&claims.sub) {
                                req.extensions_mut().insert(admin_id);
                            }
                            req.extensions_mut().insert::<Claims>(claims);
                            return Ok(next.run(req).await);
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}