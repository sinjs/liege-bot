use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
}
