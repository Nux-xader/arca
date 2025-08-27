use clap::Parser;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

/// A simple webhook server for automated deployments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The IP address to bind the server to.
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub host: IpAddr,

    /// The port to listen on.
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// The path to the configuration file.
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

pub fn parse() -> Cli {
    Cli::parse()
}
