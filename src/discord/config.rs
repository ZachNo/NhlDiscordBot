use anyhow::{Error, Result};
use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub app_id: u64,
}

pub fn read_config() -> Result<Config> {
    return Config::from_config_file("config.toml").map_err(Error::msg);
}
