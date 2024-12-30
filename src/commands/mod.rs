use serenity::all::{CreateCommand, CreateInteractionResponse, ResolvedOption};

use crate::error::Error;

pub mod math;

pub trait Command {
    async fn run<'a>(options: &'a [ResolvedOption<'_>])
        -> Result<CreateInteractionResponse, Error>;
    fn register() -> CreateCommand;
}
