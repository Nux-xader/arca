use crate::state::{AppState, WebhookPayload};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{HeaderMap, Response},
};
use std::path::Path;
use std::sync::Arc;
use tokio::process::Command;
use tracing::{error, info, warn};

// Execute deployment script asynchronously
fn execute_script(script_path: String, repo_info: String) {
    tokio::spawn(async move {
        let script_dir = match Path::new(&script_path).parent() {
            Some(path) => path,
            None => {
                error!(
                    "Could not determine parent directory for script: {}",
                    script_path
                );
                return;
            }
        };

        info!(
            "Executing deployment script: {} for {}",
            script_path, repo_info
        );

        match Command::new("sh")
            .arg("-c")
            .arg(&script_path)
            .current_dir(script_dir)
            .spawn()
        {
            Ok(mut child) => {
                if let Ok(status) = child.wait().await {
                    if status.success() {
                        info!("Deployment script completed successfully for {}", repo_info);
                    } else {
                        error!(
                            "Deployment script failed with status: {} for {}",
                            status, repo_info
                        );
                    }
                } else {
                    error!("Failed to wait for deployment script for {}", repo_info);
                }
            }
            Err(e) => {
                error!(
                    "Failed to spawn deployment script '{}': {} for {}",
                    script_path, e, repo_info
                );
            }
        }
    });
}

// Parse webhook payload and extract repository info without allocation
fn parse_webhook_info(payload: &WebhookPayload) -> Option<(&str, &str)> {
    let branch = payload.ref_name.strip_prefix("refs/heads/")?;
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
                "Found matching config for {}, executing script: {}",
                config_key, deploy_entry.script
            );
            execute_script(deploy_entry.script.clone(), config_key);
        } else {
            info!("No matching config found for: {}", config_key);
        }
    } else {
        warn!("Failed to parse webhook payload");
    }

    Ok(Response::new(Body::from("ok")))
}
