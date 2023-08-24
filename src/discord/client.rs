use serenity::prelude::{
    Client,
    GatewayIntents,
};
use crate::discord::{
    config::read_config,
    event_handler::Handler,
};

pub async fn create_client() -> Client {
    let config = read_config();
    return Client::builder(config.token, GatewayIntents::empty())
        .event_handler(Handler)
        .application_id(config.app_id)
        .await
        .expect("Error creating client");
}