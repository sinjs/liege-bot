use std::sync::Arc;

use anyhow::anyhow;
use serenity::all::{CommandInteraction, CreateCommand};

use crate::{error::Error, AppState};

mod ai;
mod code;
mod math;

pub trait CommandHandler {
    async fn handle_command(
        interaction: CommandInteraction,
        state: Arc<AppState>,
    ) -> Result<(), Error>;
    fn command() -> CreateCommand;
}

pub async fn handle_interaction(
    interaction: CommandInteraction,
    state: Arc<AppState>,
) -> Result<(), Error> {
    println!("{:?}", interaction.data);

    match interaction.data.name.as_str() {
        "math" => MathCommand::handle_command(interaction.clone(), state.clone()).await,
        "ai" => AiCommand::handle_command(interaction.clone(), state.clone()).await,
        "code" => CodeCommand::handle_command(interaction.clone(), state.clone()).await,
        name => Err(anyhow!("Command with name '{}' not found", name)),
    }
}

pub use ai::AiCommand;
pub use code::CodeCommand;
pub use math::MathCommand;
