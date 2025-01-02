use std::sync::Arc;

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

pub use ai::AiCommand;
pub use code::CodeCommand;
pub use math::MathCommand;
