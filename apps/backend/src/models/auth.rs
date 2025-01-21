use std::sync::LazyLock;

use anyhow::Context;
use axum::{
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use serenity::all::{User, UserId};

struct Keys {
    encoding: jsonwebtoken::EncodingKey,
    decoding: jsonwebtoken::DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: jsonwebtoken::EncodingKey::from_secret(secret),
            decoding: jsonwebtoken::DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub discord_access_token: String,
    pub token_type: String,
}

impl TokenResponse {
    pub fn new(token: impl Into<String>, discord_access_token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            token_type: String::from("Bearer"),
            discord_access_token: discord_access_token.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub exp: i64,

    pub username: String,
    pub display_name: String,
    pub avatar: String,

    pub discord_access_token: String,
}

impl Claims {
    pub fn new(user: &User, token_response: &DiscordTokenResponse) -> Self {
        Self {
            exp: token_response.expires_in + chrono::Utc::now().timestamp(),
            sub: user.id,
            username: user.name.clone(),
            avatar: user.avatar_url().unwrap_or(user.default_avatar_url()),
            display_name: user.global_name.clone().unwrap_or(user.name.clone()),
            discord_access_token: token_response.access_token.clone(),
        }
    }

    pub fn from_token(token: &str) -> Result<Self, crate::Error> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &KEYS.decoding,
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "missing token").into_response())?;

        let claims = Claims::from_token(bearer.token()).map_err(|_| {
            (StatusCode::UNAUTHORIZED, "token expired or invalidated").into_response()
        })?;

        Ok(claims)
    }
}

impl TryFrom<Claims> for TokenResponse {
    type Error = crate::Error;

    fn try_from(value: Claims) -> Result<Self, Self::Error> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &value, &KEYS.encoding)
            .context("Failed to create token")?;

        Ok(TokenResponse::new(token, value.discord_access_token))
    }
}
