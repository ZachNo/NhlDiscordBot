use anyhow::Result;
use serenity::all::{
    async_trait, CommandInteraction, ComponentInteraction, CreateActionRow,
    CreateAutocompleteResponse, CreateCommand, CreateEmbed,
};

#[async_trait]
pub trait DiscordCommandTrait: Sync + Send {
    fn create_command(&self) -> CreateCommand;
    async fn handle_command(
        &self,
        _command: &CommandInteraction,
    ) -> Result<(CreateEmbed, Vec<CreateActionRow>)>;
    async fn handle_autocomplete(
        &self,
        _autocomplete: &CommandInteraction,
    ) -> Result<CreateAutocompleteResponse> {
        unimplemented!()
    }
    async fn handle_interaction(
        &self,
        _message: &ComponentInteraction,
    ) -> Result<(CreateEmbed, Vec<CreateActionRow>)> {
        unimplemented!()
    }
}
