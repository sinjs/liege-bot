use std::sync::Arc;

use axum::extract::State;

use crate::AppState;

pub async fn post(State(state): State<Arc<AppState>>) {}
