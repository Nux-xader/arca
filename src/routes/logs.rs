use axum::{
    body::Body,
    extract::Path,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use futures_util::StreamExt;
use std::{convert::Infallible, future};
use tokio::process::Command;
use tokio_util::codec::{FramedRead, LinesCodec};

pub async fn handler(
    Path((service_name, lines)): Path<(String, i16)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut cmd = Command::new("pm2");
    cmd.arg("logs")
        .arg("--lines")
        .arg(lines.to_string())
        .arg(service_name)
        .arg("--raw");

    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to spawn command: {}", e),
            )
        })?;

    let stdout = child.stdout.take().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get stdout".to_string(),
        )
    })?;

    let reader = FramedRead::new(stdout, LinesCodec::new());

    let stream = reader
        .filter_map(|result| {
            // This will log the error and filter out the item, making the stream infallible.
            future::ready(
                result
                    .map_err(|e| eprintln!("Error reading line from stdout: {}", e))
                    .ok(),
            )
        })
        .map(|line| Ok::<_, Infallible>(format!("{}\n", line).into_bytes()));

    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(body)
        .unwrap())
}