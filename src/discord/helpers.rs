use anyhow::{anyhow, Error, Result};
use serenity::model::application::interaction::application_command::CommandData;

pub async fn get_match_id(data: &CommandData) -> Result<u64> {
    let option = data
        .options
        .iter()
        .find(|x| x.name == "match")
        .ok_or(anyhow!("Cannot find match"))?
        .value
        .as_ref()
        .unwrap()
        .to_string();
    let trimmed_option = option.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
    trimmed_option.parse::<u64>().map_err(Error::msg)
}
