use crate::config::DeployEntry;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use utoipa::ToSchema;

#[derive(Clone, Default)]
pub struct AppState {
    pub expected_token: String,
    pub config: Arc<RwLock<HashMap<String, DeployEntry>>>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WebhookPayload {
    #[serde(rename = "ref")]
    pub ref_name: Option<String>,
    pub repository: Repository,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct Repository {
    pub full_name: String,
}
