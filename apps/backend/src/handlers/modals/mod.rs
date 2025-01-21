use std::sync::Arc;

use crate::{error::Error, models::custom_id::CustomId, AppState};
use anyhow::anyhow;
use serenity::all::{CreateModal, ModalInteraction};

mod code;

pub trait ModalHandler {
    async fn handle_modal(interaction: ModalInteraction, state: Arc<AppState>)
        -> Result<(), Error>;
    fn modal(data: Option<Vec<String>>) -> CreateModal;
}

pub async fn handle_interaction(
    interaction: ModalInteraction,
    state: Arc<AppState>,
) -> Result<(), Error> {
    let custom_id = CustomId::try_from(interaction.data.custom_id.clone())?;

    match custom_id.id.as_ref() {
        "code" => CodeModal::handle_modal(interaction.clone(), state.clone()).await,
        name => Err(anyhow!("Modal with ID '{}' not found", name)),
    }
}

pub use code::CodeModal;
