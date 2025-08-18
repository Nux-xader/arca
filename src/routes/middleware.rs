use crate::state::AppState;
use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::{collections::HashMap, sync::Arc};

// Middleware for token validation
pub async fn validate_token(
    Query(query): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(token) = query.get("token") {
        if token == &state.expected_token {
            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}