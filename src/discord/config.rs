use anyhow::{Context, Result};
use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub app_id: u64,
    pub error_channel: Option<u64>,
    pub influxdb_endpoint: Option<String>,
    pub influxdb_database: Option<String>,
    pub influxdb_username: Option<String>,
    pub influxdb_password: Option<String>,
}

pub fn read_config() -> Result<Config> {
    Config::from_config_file("config.toml").context("config read")
}
