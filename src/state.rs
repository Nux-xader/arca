use crate::config::DeployEntry;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub expected_token: String,
    pub config: Arc<RwLock<HashMap<String, DeployEntry>>>,
}

#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
}