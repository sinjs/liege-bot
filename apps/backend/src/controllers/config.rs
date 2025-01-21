use std::env;

use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    discord_app_id: String,
}

pub async fn get() -> Json<ConfigResponse> {
    let discord_app_id =
        env::var("DISCORD_APP_ID").expect("Missing DISCORD_APP_ID environment variable");

    Json(ConfigResponse { discord_app_id })
}
