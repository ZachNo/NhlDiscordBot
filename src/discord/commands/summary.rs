use anyhow::Result;
use serenity::all::{
    async_trait, Colour, CommandInteraction, CreateActionRow, CreateCommand, CreateEmbed,
};

use crate::discord::commands::DiscordCommandTrait;
use crate::nhl::fetch_data::{fetch_team_name, fetch_yesterday_schedule};
use crate::nhl::model::schedule::Day;

pub const NAME: &str = "summary";
const DESCRIPTION: &str = "Print the summary of yesterday's games";

pub struct Summary {}

#[async_trait]
impl DiscordCommandTrait for Summary {
    fn create_command(&self) -> CreateCommand {
        CreateCommand::new(NAME).description(DESCRIPTION)
    }

    async fn handle_command(
        &self,
        _command: &CommandInteraction,
    ) -> Result<(CreateEmbed, Vec<CreateActionRow>)> {
        Ok((
            format_summary(fetch_yesterday_schedule().await?).await?,
            vec![],
        ))
    }
}

async fn format_summary(schedule: Day) -> Result<CreateEmbed> {
    if schedule.games.is_empty() {
        let embed: CreateEmbed = CreateEmbed::default()
            .color(Colour::from_rgb(240, 200, 0))
            .title(format!("NHL Games Summary for {}", &schedule.date))
            .description("There were no games. :c");
        return Ok(embed);
    }

    let mut embed: CreateEmbed = CreateEmbed::default()
        .color(Colour::from_rgb(240, 200, 0))
        .title(format!("NHL Games Summary for {}", &schedule.date));
    for game in &schedule.games {
        let away_team_name = fetch_team_name(game.away_team.id).await?;
        let home_team_name = fetch_team_name(game.home_team.id).await?;
        embed = embed.field(
            format!("{} vs. {}", home_team_name, away_team_name),
            format!(
                "Final score: {}-{}",
                game.home_team.score.unwrap_or(0),
                game.away_team.score.unwrap_or(0),
            ),
            true,
        );
    }

    Ok(embed)
}
