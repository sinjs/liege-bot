use std::{collections::HashMap, env, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::token::{DiscordTokenResponse, TokenRequest},
    AppState,
};

#[axum::debug_handler]
pub async fn post(State(state): State<Arc<AppState>>, Json(body): Json<TokenRequest>) -> Response {
    match get_discord_oauth_token(&state, &body.code).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(_error) => (StatusCode::INTERNAL_SERVER_ERROR, "failed to get token").into_response(),
    }
}

async fn get_discord_oauth_token(
    state: &Arc<AppState>,
    code: &str,
) -> Result<DiscordTokenResponse, Error> {
    let client_id = env::var("DISCORD_APP_ID").unwrap();
    let client_secret = env::var("DISCORD_CLIENT_SECRET").unwrap();

    let mut form: HashMap<&str, &str> = HashMap::new();
    form.insert("client_id", &client_id);
    form.insert("client_secret", &client_secret);
    form.insert("grant_type", "authorization_code");
    form.insert("code", &code);

    let response = state
        .http_client
        .post("https://discord.com/api/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await?
        .error_for_status()?
        .json::<DiscordTokenResponse>()
        .await?;

    Ok(response)
}
