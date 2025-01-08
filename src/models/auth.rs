use std::sync::LazyLock;

use anyhow::Context;
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
    pub token_type: String,
}

impl TokenResponse {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            token_type: String::from("Bearer"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub exp: usize,
}

impl Claims {
    pub fn new(user: &User, token_response: &DiscordTokenResponse) -> Self {
        Self {
            exp: token_response.expires_in,
            sub: user.id,
        }
    }
}

impl TryFrom<Claims> for TokenResponse {
    type Error = crate::Error;

    fn try_from(value: Claims) -> Result<Self, Self::Error> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &value, &KEYS.encoding)
            .context("Failed to create token")?;

        Ok(TokenResponse::new(token))
    }
}
