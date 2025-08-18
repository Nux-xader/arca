use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};

// Echo endpoint handler
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

    Response::new(Body::from(response))
}