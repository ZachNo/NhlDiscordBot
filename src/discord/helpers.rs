use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction,
    CommandData,
};

pub async fn get_match_id(data: &CommandData) -> u64 {
    let option = data.options
        .iter().find(|x| x.name == "match").unwrap()
        .value.as_ref().unwrap()
        .to_string();
    let trimmed_option = option
        .strip_prefix('"').unwrap()
        .strip_suffix('"').unwrap();
    trimmed_option.parse::<u64>().unwrap()
}

pub async fn get_highlight_id_from_input(command: &ApplicationCommandInteraction) -> u64 {
    let option = command.data.options
        .iter().find(|x| x.name == "highlight").unwrap()
        .value.as_ref().unwrap()
        .to_string();
    let trimmed_option = option
        .strip_prefix('"').unwrap()
        .strip_suffix('"').unwrap();
    trimmed_option.parse::<u64>().unwrap()
}