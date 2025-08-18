use crate::state::AppState;
use axum::{
    middleware as axum_middleware,
    routing::{any, post},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod deploy;
pub mod echo;
pub mod middleware;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/echo", any(echo::handler))
        .route("/api/deploy", post(deploy::handler))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(axum_middleware::from_fn_with_state(
                    state.clone(),
                    middleware::validate_token,
                )),
        )
        .with_state(state)
}