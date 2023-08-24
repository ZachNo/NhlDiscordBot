use serenity::{
    builder::CreateApplicationCommands,
    model::application::command::CommandOptionType,
};

pub fn application_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(|command| {
            command
                .name("schedule")
                .description("Print the schedule for today's games")
        })
        .create_application_command(|command| {
            command
                .name("score")
                .description("Print the live score for a match")
                .create_option(|option| {
                    option
                        .name("match")
                        .description("Which match to view the score of.")
                        .required(true)
                        .kind(CommandOptionType::String)
                        .set_autocomplete(true)
                })
        })
        .create_application_command(|command| {
            command
                .name("highlight")
                .description("Posts a match highlight")
                .create_option(|option| {
                    option
                        .name("match")
                        .description("Which match to view highlights of.")
                        .required(true)
                        .kind(CommandOptionType::String)
                        .set_autocomplete(true)
                })
                .create_option(|option| {
                    option
                        .name("highlight")
                        .description("Which highlight to view.")
                        .required(true)
                        .kind(CommandOptionType::String)
                        .set_autocomplete(true)
                })
        })
}

