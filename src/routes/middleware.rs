use crate::state::AppState;
use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use std::{collections::HashMap, sync::Arc};

// Middleware for token validation
pub async fn check_auth(
    jar: CookieJar,
    Query(query): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = jar
        .get("auth-token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| query.get("token").cloned());

    if let Some(token) = token {
        if token == state.expected_token {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
