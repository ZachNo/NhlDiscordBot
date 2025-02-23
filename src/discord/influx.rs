use crate::discord::config::Config;
#[cfg(feature = "influxdbstats")]
use chrono::Local;
#[cfg(feature = "influxdbstats")]
use influxdb::{Client, WriteQuery};

pub async fn send_usage_metric(config: &Config, interaction_type: String, command: String) {
    #[cfg(not(feature = "influxdbstats"))]
    return;

    #[cfg(feature = "influxdbstats")]
    {
        if config.influxdb_endpoint.is_none() || config.influxdb_database.is_none() {
            return;
        }

        let mut client = Client::new(
            config.influxdb_endpoint.clone().unwrap(),
            config.influxdb_database.clone().unwrap(),
        );
        if config.influxdb_username.is_some() && config.influxdb_password.is_some() {
            client = client.with_auth(
                config.influxdb_username.clone().unwrap(),
                config.influxdb_password.clone().unwrap(),
            );
        }

        let write = WriteQuery::new(Local::now().into(), "nhl_bot")
            .add_tag("interaction", interaction_type)
            .add_tag("name", command)
            .add_field("count", 1);

        match client.query(write).await {
            Ok(_) => {}
            Err(e) => println!("{e:?}"),
        };
    }
}
