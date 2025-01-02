use std::sync::Arc;

use crate::{error::Error, AppState};
use serenity::all::{CreateModal, ModalInteraction};

mod code;

pub trait ModalHandler {
    async fn handle_modal(interaction: ModalInteraction, state: Arc<AppState>)
        -> Result<(), Error>;
    fn modal(data: Option<Vec<String>>) -> CreateModal;
}

pub use code::CodeModal;
