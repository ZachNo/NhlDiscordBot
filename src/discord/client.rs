use crate::discord::{config::read_config, event_handler::Handler};
use anyhow::{Error, Result};
use serenity::prelude::{Client, GatewayIntents};

pub async fn create_client() -> Result<Client> {
    let config = read_config()?;
    return Client::builder(config.token, GatewayIntents::empty())
        .event_handler(Handler)
        .application_id(config.app_id)
        .await
        .map_err(Error::msg);
}
