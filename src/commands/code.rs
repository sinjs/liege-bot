use std::sync::Arc;

use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, InstallationContext,
    InteractionContext,
};

use crate::{error::Error, AppState};

use super::Command;

pub struct CodeCommand;

impl Command for CodeCommand {
    async fn run(interaction: CommandInteraction, state: Arc<AppState>) -> Result<(), Error> {
        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("code")
            .description("Execute code")
            .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
            .contexts(vec![
                InteractionContext::Guild,
                InteractionContext::BotDm,
                InteractionContext::PrivateChannel,
            ])
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "language",
                    "Programming language to use",
                )
                .required(true)
                .add_string_choice("JavaScript", "js")
                .add_string_choice("Rust", "rust")
                .add_string_choice("Python", "python"),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "code", "The code to execute")
                    .required(false),
            )
    }
}
