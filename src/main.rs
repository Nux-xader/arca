use arca::{
    cli,
    config::{self, Config},
    error::Error,
    routes,
    state::AppState,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let args = cli::parse();
    let config_path = args.config;
    let config: Config = config::load_config(config_path.clone()).await?;
    info!("Configuration loaded successfully");

    let expected_token = config.secret_key.ok_or(Error::MissingKey("secret_key"))?;

    let reload_interval = config
        .reload_interval
        .ok_or(Error::MissingKey("reload_interval"))?;
    info!("Using reload interval: {} seconds", reload_interval);

    let deploy_config = Arc::new(RwLock::new(config.deploy));

    let state = Arc::new(AppState {
        expected_token,
        config: deploy_config.clone(),
    });

    tokio::spawn(async move {
        config::config_reload_task(deploy_config, reload_interval, config_path.clone()).await;
    });

    let app = routes::create_router(state);
    let addr = SocketAddr::new(args.host, args.port);
    let listener = TcpListener::bind(addr).await?;
    info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await.map_err(Error::Io)?;

    Ok(())
}
