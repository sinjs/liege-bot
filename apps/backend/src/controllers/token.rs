use std::{collections::HashMap, env, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serenity::all::User;

use crate::{
    error::Error,
    models::auth::{Claims, DiscordTokenResponse, TokenRequest, TokenResponse},
    AppState,
};

pub async fn post(State(state): State<Arc<AppState>>, Json(body): Json<TokenRequest>) -> Response {
    let Ok(token_response) = get_discord_oauth_token(&state, &body.code).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed to get token").into_response();
    };

    let Ok(user) = get_user_from_token(&state, &token_response.access_token).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed to get user").into_response();
    };

    let claims = Claims::new(&user, &token_response);

    let Ok(response) = TokenResponse::try_from(claims) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed to create token").into_response();
    };

    (StatusCode::OK, Json(response)).into_response()
}

pub async fn get(claims: Claims) -> Response {
    (StatusCode::OK, Json(claims)).into_response()
}

async fn get_user_from_token(state: &Arc<AppState>, token: &str) -> Result<User, Error> {
    let response = state
        .http_client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<User>()
        .await?;

    Ok(response)
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
