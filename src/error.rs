use anyhow::Error as AnyhowError;
use chrono::Local;
use serenity::all::{Context, CreateMessage};
use thiserror::Error;

pub const NON_USER_ERROR_OUTPUT: &str = "Unexpected error occurred, the issue has been reported.";

#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("No value given")]
    NoValueProvided,
    #[error("No match found for {0}. Please use the autocomplete list")]
    NoMatchFound(String),
    #[error("Error getting data from the NHL")]
    NhlServerError(String),
    #[error("No command found for {0}")]
    InvalidCommand(String),
}

pub async fn error_to_error_message(err: AnyhowError, context: &Context, error_channel: Option<u64>) -> String {
    println!("{} {err:#}", Local::now().format("%Y-%m-%dT%H:%M:%S"));
    if let Some(err_chan) = error_channel {
        let builder = CreateMessage::new()
            .content(format!("{err:#}"));
        context.http.send_message(err_chan.into(), vec![], &builder).await.unwrap();
    }
    if let Some(e) = err.downcast_ref::<DiscordError>() {
        format!("{e}")
    } else {
        NON_USER_ERROR_OUTPUT.to_string()
    }
}
