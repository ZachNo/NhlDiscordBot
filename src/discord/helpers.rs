use crate::error::DiscordError::{NoMatchFound, NoValueProvided};
use anyhow::{anyhow, Result};
use serenity::model::application::CommandData;

pub async fn get_match_id(data: &CommandData) -> Result<u64> {
    let data = data
        .options
        .iter()
        .find(|x| x.name == "match")
        .ok_or(anyhow!("Cannot find match"))?
        .value
        .as_str()
        .ok_or(NoValueProvided)?;
    data.parse::<u64>()
        .map_err(|_| NoMatchFound(data.to_string()).into())
}
