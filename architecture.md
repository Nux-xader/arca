# Frontend Architecture Design: Serving Static Files from Rust Backend

## Objective

This document outlines the architectural changes required to replace the existing login mechanism with a client-side solution. The Rust backend will serve static frontend files (HTML, JS), and the frontend will handle token prompting and cookie storage, eliminating the need for a dedicated login route.

## Proposed Directory Structure

A new top-level directory named `static` will be created to house all frontend assets. This directory will contain `index.html`, `main.js`, `style.css`, and any other static files required by the frontend.

```
.
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── routes/
│       ├── mod.rs
│       └── index.rs (will be modified or removed)
└── static/
    ├── index.html
    ├── main.js
    └── style.css
```

## Required Backend Modifications (Axum)

The following sections detail the necessary changes in the Rust backend to serve the static files and manage the new frontend-driven authentication flow.

### 1. `Cargo.toml` Changes

The `tower-http` dependency needs to have the `fs` feature enabled to allow serving static files.

```toml
[dependencies]
# ... other dependencies ...
tower-http = { version = "0.5.2", features = ["cors", "fs"] } # Add "fs" feature
# ...
```

### 2. `src/routes/mod.rs` Modifications

The `create_router` function in `src/routes/mod.rs` will be modified to include a service that serves static files from the `static` directory. The `index.html` file will be served as the main entry point for the root URL (`/`).

The `tower-http::services::ServeDir` will be used for this purpose. It will be configured to serve `index.html` when the root path is requested.

```rust
use crate::state::AppState;
use axum::{
    Router, middleware as axum_middleware,
    routing::{any, post}, // Removed `get` for index_page
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir; // New import
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::{Repository, WebhookPayload};

// ... ApiDoc struct ...

pub mod deploy;
pub mod echo;
pub mod middleware;
// pub mod index; // This module will be removed or significantly refactored

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .route("/api/echo", any(echo::handler))
        .route("/api/deploy", post(deploy::handler))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()).layer(
            axum_middleware::from_fn_with_state(state.clone(), middleware::check_auth),
        ));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Serve static files from the "static" directory, with index.html as the fallback
        .nest_service("/", ServeDir::new("static").fallback(ServeDir::new("static").append_index_html_on_directories(true)))
        // .route("/", get(index::index_page)) // This route will be removed
        // .route("/auth/login", post(index::login_handler)) // This route will be removed
        .merge(api_routes)
        .with_state(state)
}
```

### 3. Removal of Existing Login Route and Handler

The existing login mechanism, including the `/auth/login` route and the `index::login_handler` function, will be removed. The `index::index_page` function will also be removed or refactored as its responsibility will now be handled by serving `static/index.html`.

Specifically:
- Remove `pub mod index;` from `src/routes/mod.rs`.
- Delete or completely refactor `src/routes/index.rs`. The logic for token prompting and cookie storage will be handled by the client-side JavaScript in `static/main.js`.

### 4. Frontend (Client-side) Authentication

The `static/index.html` will be the main entry point. A `static/main.js` file will contain the client-side logic for:
- Prompting the user for a token (e.g., using `prompt()`).
- Storing the token securely in a browser cookie.
- Attaching the token to subsequent API requests (e.g., in an `Authorization` header or as a cookie).

This approach shifts the authentication UI and token management entirely to the client, simplifying the backend and improving flexibility.

## Architecture Diagram

```mermaid
graph TD
    A[Client Browser] -->|Requests /| B(Rust Backend)
    B -->|Serves static/index.html| A
    A -->|index.html loads main.js| A
    A -->|main.js prompts for token| A
    A -->|main.js sets cookie| A
    A -->|Subsequent API Requests with Token| B
    B -->|API Routes (e.g., /api/deploy)| B
```

This design provides a clear separation of concerns, with the Rust backend focusing on API services and static file serving, and the frontend handling user interaction and client-side authentication.