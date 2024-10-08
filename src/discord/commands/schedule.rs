use anyhow::Result;
use chrono::DateTime;
use futures::future::join_all;
use serenity::all::{
    async_trait, Colour, CommandInteraction, CommandOptionType, CreateActionRow, CreateCommand,
    CreateCommandOption, CreateEmbed,
};

use crate::discord::commands::DiscordCommandTrait;
use crate::nhl::fetch_data::{fetch_team_name, fetch_today_schedule, fetch_tomorrow_schedule};
use crate::nhl::model::schedule::Day;
use crate::nhl::utils::translate_match_status;

pub const NAME: &str = "schedule";
const DESCRIPTION: &str = "Print the schedule for today's games";

pub struct Schedule {}

#[async_trait]
impl DiscordCommandTrait for Schedule {
    fn create_command(&self) -> CreateCommand {
        CreateCommand::new(NAME)
            .description(DESCRIPTION)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "day",
                    "Which schedule to display?",
                )
                .add_string_choice("today", "today")
                .add_string_choice("tomorrow", "tomorrow"),
            )
    }

    async fn handle_command(
        &self,
        command: &CommandInteraction,
    ) -> Result<(CreateEmbed, Vec<CreateActionRow>)> {
        if command
            .data
            .options
            .iter()
            .filter(|x| x.name == "day" && x.value.as_str().is_some_and(|y| y == "tomorrow"))
            .count()
            > 0
        {
            Ok((
                format_schedule(fetch_tomorrow_schedule().await?).await?,
                vec![],
            ))
        } else {
            Ok((
                format_schedule(fetch_today_schedule().await?).await?,
                vec![],
            ))
        }
    }
}

async fn format_schedule(schedule: Day) -> Result<CreateEmbed> {
    if schedule.games.is_empty() {
        let embed: CreateEmbed = CreateEmbed::default()
            .color(Colour::from_rgb(240, 200, 0))
            .title(format!("NHL Games for {}", &schedule.date))
            .description("There's no games scheduled. :c");
        return Ok(embed);
    }

    let mut embed: CreateEmbed = CreateEmbed::default()
        .color(Colour::from_rgb(240, 200, 0))
        .title(format!("NHL Games for {}", &schedule.date));

    // Make sure all teams are cached
    // Fetch each team in parallel, and wait for all done
    let mut handles = vec![];
    schedule
        .games
        .iter()
        .for_each(|g| {
            handles.push(fetch_team_name(g.away_team.id));
            handles.push(fetch_team_name(g.home_team.id));
        });
    join_all(handles).await;

    for game in &schedule.games {
        let datetime = DateTime::parse_from_rfc3339(game.start_time_u_t_c.as_str())?;
        let away_team_name = fetch_team_name(game.away_team.id).await?;
        let home_team_name = fetch_team_name(game.home_team.id).await?;
        embed = embed.field(
            format!("{} vs. {}", home_team_name, away_team_name),
            format!(
                "At {} @ <t:{}:t>\n{}{}",
                game.venue.default,
                datetime.timestamp(),
                translate_match_status(&game.game_state),
                if game.special_event.is_some() {
                    format!(
                        "\nSpecial Event: {}",
                        game.special_event.clone().unwrap().default
                    )
                } else {
                    "".to_string()
                }
            ),
            true,
        );
    }

    Ok(embed)
}
