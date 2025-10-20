use std::net::IpAddr;

use axum::http::Request;
use serenity::all::UserId;
use tower_governor::{
    GovernorError,
    key_extractor::{KeyExtractor, SmartIpKeyExtractor},
};

use crate::models::auth::Claims;

#[derive(Clone)]
pub struct JwtKeyExtractor;

impl JwtKeyExtractor {
    fn get_token_from_response<T>(req: &Request<T>) -> Option<&str> {
        let headers = req.headers();

        let token = headers
            .get("Authorization")?
            .to_str()
            .ok()?
            .strip_prefix("Bearer ")?;

        Some(token)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UserIpKey {
    user_id: Option<UserId>,
    ip_addr: Option<IpAddr>,
}

impl From<UserId> for UserIpKey {
    fn from(user_id: UserId) -> Self {
        Self {
            user_id: Some(user_id),
            ip_addr: None,
        }
    }
}

impl From<IpAddr> for UserIpKey {
    fn from(ip_addr: IpAddr) -> Self {
        Self {
            user_id: None,
            ip_addr: Some(ip_addr),
        }
    }
}

impl KeyExtractor for JwtKeyExtractor {
    type Key = UserIpKey;

    fn extract<T>(
        &self,
        req: &axum::http::Request<T>,
    ) -> Result<Self::Key, tower_governor::GovernorError> {
        let token = Self::get_token_from_response(req);

        match token {
            Some(token) => {
                let token_data =
                    Claims::from_token(token).map_err(|_| GovernorError::UnableToExtractKey);

                match token_data {
                    Ok(token_data) => Ok(UserIpKey::from(token_data.sub)),
                    Err(_) => SmartIpKeyExtractor
                        .extract(req)
                        .map(|ip| UserIpKey::from(ip)),
                }
            }
            None => {
                let result = SmartIpKeyExtractor
                    .extract(req)
                    .map(|ip| UserIpKey::from(ip));
                tracing::warn!("result {:?}", result);
                result
            }
        }
    }
}
