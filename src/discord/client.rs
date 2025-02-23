use crate::discord::{config::read_config, event_handler::Handler};
use anyhow::{Context, Result};
use serenity::prelude::{Client, GatewayIntents};

pub async fn create_client() -> Result<Client> {
    let config = read_config()?;
    Client::builder(config.token.clone(), GatewayIntents::empty())
        .application_id(config.app_id.into())
        .event_handler(Handler {config})
        .await
        .context("create_client")
}
