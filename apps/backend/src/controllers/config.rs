use axum::Json;
use serde::{Deserialize, Serialize};

use crate::env::ENV;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    discord_app_id: String,
}

pub async fn get() -> Json<ConfigResponse> {
    let discord_app_id = ENV.discord_app_id.clone();

    Json(ConfigResponse { discord_app_id })
}
