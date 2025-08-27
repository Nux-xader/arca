use crate::error::Error;
use serde::Deserialize;
use std::{collections::HashMap, io, path::PathBuf, sync::Arc, time::Duration};
use tokio::{
    fs,
    sync::RwLock,
    time::{MissedTickBehavior, interval},
};
use tracing::{error, info, warn};

// --- Configuration Structs ---
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub secret_key: Option<String>,
    pub reload_interval: Option<u64>,
    pub deploy: HashMap<String, DeployEntry>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct DeployEntry {
    pub script: String,
}

// Load configuration from TOML file
pub async fn load_config(config_path: PathBuf) -> Result<Config, Error> {
    match fs::read_to_string(&config_path).await {
        Ok(content) => {
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            warn!("Config file not found, using empty config");
            Ok(Config {
                secret_key: None,
                reload_interval: None,
                deploy: HashMap::new(),
            })
        }
        Err(e) => Err(Error::Io(e.to_string())),
    }
}

// Config reload task that runs at a specified interval
pub async fn config_reload_task(
    deploy_config: Arc<RwLock<HashMap<String, DeployEntry>>>,
    reload_interval: u64,
    config_path: PathBuf,
) {
    let duration = Duration::from_secs(reload_interval.max(1));
    let mut interval = interval(duration);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        interval.tick().await;
        match load_config(config_path.clone()).await {
            Ok(new_config) => {
                let mut config_guard = deploy_config.write().await;
                if *config_guard != new_config.deploy {
                    *config_guard = new_config.deploy;
                    info!("Configuration reloaded successfully");
                }
            }
            Err(e) => {
                error!("Failed to reload configuration: {:?}", e);
            }
        }
    }
}
