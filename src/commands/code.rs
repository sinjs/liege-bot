use std::sync::Arc;

use serenity::all::{CommandInteraction, CreateCommand};

use crate::{error::Error, AppState};

use super::Command;

pub struct CodeCommand;

impl Command for CodeCommand {
    async fn run(interaction: CommandInteraction, state: Arc<AppState>) -> Result<(), Error> {}

    fn register() -> CreateCommand {
        CreateCommand::new("code")
    }
}
