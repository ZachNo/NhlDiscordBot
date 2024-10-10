pub fn translate_match_status<'a>(game_state: &String) -> &'a str {
    match game_state.as_str() {
        "OFF" => "Finished",
        "FINAL" => "Finished",
        "OVER" => "Finished",
        "LIVE" => "In Progress",
        "PRE" => "Pre-game",
        "FUT" => "Scheduled",
        "CRIT" => "In Progress",
        _ => {
            println!("Unknown game state: {game_state}");
            "Unknown"
        }
    }
}
