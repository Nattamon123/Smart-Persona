use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;

use crate::{
    config::{config_loader::get_stage, stage::Stage},
    domain::repo::user::UserRepository,
    infrastructure::{
        jwt_authentication::authentication_model::LoginModel,
        postgres::{
            postgres_connection::DbPool,
            repositories::user::UserPostgres,
        },
    },
};

use crate::domain::usecase::authentication::AuthenticationUseCase;

pub fn routes(db_pool: Arc<DbPool>) -> Router {
    let user_repository = UserPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(Arc::new(user_repository));

    Router::new()
        .route("/login", post(user_login::<UserPostgres>))
        .route("/refresh-token", post(user_refresh_token::<UserPostgres>))
        .route("/admin/login", post(admin_login::<UserPostgres>))
        .route("/admin/refresh-token", post(admin_refresh_token::<UserPostgres>))
        .with_state(Arc::new(authentication_use_case))
}

pub async fn user_login<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    match authentication_use_case.user_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                rft_cookie = rft_cookie.secure(true);
                act_cookie = act_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn user_refresh_token<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_use_case
            .user_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    rft_cookie = rft_cookie.secure(true);
                    act_cookie = act_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Refresh token successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };

        return response;
    }

    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}

pub async fn admin_login<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    match authentication_use_case.admin_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                rft_cookie = rft_cookie.secure(true);
                act_cookie = act_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Admin login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn admin_refresh_token<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_use_case
            .admin_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    rft_cookie = rft_cookie.secure(true);
                    act_cookie = act_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Admin refresh token successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };

        return response;
    }

    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}