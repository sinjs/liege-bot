use std::sync::Arc;

use serenity::all::{CommandInteraction, CreateCommand, CreateInteractionResponse, ResolvedOption};

use crate::{error::Error, AppState};

mod ai;
mod math;

pub trait Command {
    async fn run(interaction: CommandInteraction, state: Arc<AppState>) -> Result<(), Error>;
    fn register() -> CreateCommand;
}

pub use ai::AiCommand;
pub use math::MathCommand;
