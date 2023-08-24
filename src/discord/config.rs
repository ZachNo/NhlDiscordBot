use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub app_id: u64,
}

pub fn read_config() -> Config {
    return Config::from_config_file("config.toml").unwrap();
}