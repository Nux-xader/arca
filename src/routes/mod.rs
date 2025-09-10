use crate::state::AppState;
use axum::{
    Router, middleware as axum_middleware,
    routing::{any, get, post},
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::WebhookPayload;

#[derive(OpenApi)]
#[openapi(
    paths(deploy::handler, echo::handler, echo::post, echo::put, echo::delete,),
    components(schemas(WebhookPayload))
)]
struct ApiDoc;

pub mod deploy;
pub mod echo;
pub mod middleware;
pub mod logs;

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new().nest(
        "/api",
        Router::new()
            .route("/echo", any(echo::handler))
            .route("/deploy", post(deploy::handler))
            .route("/logs/:service_name/:lines", get(logs::handler))
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()).layer(
                axum_middleware::from_fn_with_state(state.clone(), middleware::check_auth),
            )),
    );

    Router::new()
        .merge(api_routes)
        .merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", ApiDoc::openapi()))
        .fallback_service(ServeDir::new("public"))
        .with_state(state)
}
