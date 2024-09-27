mod common;
mod schedule;
mod score;
mod summary;

pub use crate::discord::commands::common::DiscordCommandTrait;
use strum::{EnumIter, EnumString};

#[derive(EnumIter, EnumString)]
pub enum DiscordCommand {
    #[strum(serialize = "schedule")]
    Schedule,
    #[strum(serialize = "score")]
    Score,
    #[strum(serialize = "summary")]
    Summary,
}

impl DiscordCommand {
    pub fn into_command(self) -> Box<dyn DiscordCommandTrait> {
        match self {
            DiscordCommand::Schedule => Box::new(schedule::Schedule {}),
            DiscordCommand::Score => Box::new(score::Score {}),
            DiscordCommand::Summary => Box::new(summary::Summary {}),
        }
    }
}
