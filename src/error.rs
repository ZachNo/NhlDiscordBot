use anyhow::Error as AnyhowError;
use thiserror::Error;

pub const NON_USER_ERROR_OUTPUT: &str = "Unexpected error occured, the issue has been reported.";

#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("No value given")]
    NoValueProvided,
    #[error("No match found for {0}")]
    NoMatchFound(String),
    #[error("Error getting data from the NHL")]
    NhlServerError(String),
    #[error("No command found for {0}")]
    InvalidCommand(String),
}

pub fn error_to_error_message(err: AnyhowError) -> String {
    println!("{err:#}");
    if let Some(e) = err.downcast_ref::<DiscordError>() {
        format!("{e}")
    } else {
        NON_USER_ERROR_OUTPUT.to_string()
    }
}
