use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use tracing::info;

/// Echo endpoint
#[utoipa::path(
    get,
    path = "/api/echo",
    responses(
        (status = 200, description = "Echo response", body = String)
    )
)]
pub async fn handler(req: Request<Body>) -> Response {
    let (parts, body) = req.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to read body: {}", err)))
                .unwrap();
        }
    };

    let mut response = format!(
        "Method: {}\nURI: {}\nVersion: {:?}\n",
        parts.method, parts.uri, parts.version
    );

    response.push_str("Headers:\n");
    for (key, value) in &parts.headers {
        response.push_str(&format!(
            "  {}: {}\n",
            key,
            value.to_str().unwrap_or("[invalid UTF-8]")
        ));
    }

    if let Ok(body_str) = std::str::from_utf8(&body_bytes) {
        if !body_str.is_empty() {
            response.push_str(&format!("\nBody:\n{}", body_str));
        }
    }

    info!(response);
    Response::new(Body::from(response))
}

#[utoipa::path(
    post,
    path = "/api/echo",
    tag = "echo",
    request_body(content = String, description = "Request body to echo", example = json!("This is a test.")),
    responses(
        (
            status = 200,
            description = "Echo response with body",
            body = String,
            example = json!("Method: POST\nURI: /api/echo\nVersion: HTTP/1.1\nHeaders:\n  content-type: text/plain\n\nBody:\nThis is a test.")
        )
    )
)]
pub async fn post() {}

#[utoipa::path(
    put,
    path = "/api/echo",
    tag = "echo",
    request_body(content = String, description = "Request body to echo", example = json!("This is a test.")),
    responses(
        (
            status = 200,
            description = "Echo response with body",
            body = String,
            example = json!("Method: PUT\nURI: /api/echo\nVersion: HTTP/1.1\nHeaders:\n  content-type: text/plain\n\nBody:\nThis is a test.")
        )
    )
)]
pub async fn put() {}

#[utoipa::path(
    delete,
    path = "/api/echo",
    tag = "echo",
    responses(
        (
            status = 200,
            description = "Echo response",
            body = String,
            example = json!("Method: DELETE\nURI: /api/echo\nVersion: HTTP/1.1\nHeaders:\n  host: example.com\n\n")
        )
    )
)]
pub async fn delete() {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[tokio::test]
    async fn test_echo() {
        let request = Request::builder()
            .uri("/api/echo")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = handler(request).await;

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        assert!(body_str.contains("Method: GET"));
        assert!(body_str.contains("URI: /api/echo"));
    }
}
