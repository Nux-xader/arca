use crate::state::{AppState, WebhookPayload};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{HeaderMap, Response},
};
use std::sync::Arc;
use tokio::process::Command;
use tracing::{error, info, warn};

// Parse webhook payload and extract repository info without allocation
fn parse_webhook_info(payload: &WebhookPayload) -> Option<(&str, &str)> {
    let ref_name = payload.ref_name.as_ref()?;
    let branch = ref_name.strip_prefix("refs/heads/")?;
    let repo = &payload.repository.full_name;
    Some((repo, branch))
}

/// Deploy endpoint handler for POST requests (webhooks)
#[utoipa::path(
    post,
    path = "/api/deploy",
    request_body = WebhookPayload,
    responses(
        (status = 200, description = "Deployment started successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<WebhookPayload>,
) -> Result<Response<Body>, axum::http::StatusCode> {
    if let Some(event) = headers.get("X-GitHub-Event") {
        if event == "ping" {
            info!("Received GitHub ping event");
            return Ok(Response::new(Body::empty()));
        }
    }

    info!("Received webhook payload: {:?}", payload);

    if let Some((repo, branch)) = parse_webhook_info(&payload) {
        let config_key = format!("{}#{}", repo, branch);
        info!("Looking for config entry: {}", config_key);

        let deploy_config = state.config.read().await;
        if let Some(deploy_entry) = deploy_config.get(&config_key) {
            info!(
                "Found matching config for {}, executing command: {}",
                config_key, deploy_entry.service_name
            );

            let output_result = Command::new("pm2")
                .arg("restart")
                .arg(deploy_entry.service_name.clone())
                .output()
                .await;

            match output_result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if output.status.success() {
                        if !stdout.is_empty() {
                            info!("stdout:\n{}", stdout);
                        }
                        if !stderr.is_empty() {
                            info!("stderr:\n{}", stderr);
                        }
                    } else {
                        error!(
                            "Failed restart service {}, with status: {}",
                            deploy_entry.service_name, output.status
                        );
                        if !stdout.is_empty() {
                            error!("stdout:\n{}", stdout);
                        }
                        if !stderr.is_empty() {
                            error!("stderr:\n{}", stderr);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed restart {} : {}", deploy_entry.service_name, e);
                }
            }
        } else {
            info!("No matching config found for: {}", config_key);
        }
    } else {
        warn!("Failed to parse webhook payload");
    }

    Ok(Response::new(Body::from("ok")))
}
