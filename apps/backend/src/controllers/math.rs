use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{math, models::auth::Claims};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MathRequest {
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MathResponse {
    pub success: bool,
    pub output: String,
}

pub async fn post(_claims: Claims, Json(body): Json<MathRequest>) -> Json<MathResponse> {
    let result = math::evaluate_html(&body.input);
    let success = result.is_ok();
    let output = result.unwrap_or_else(|o| o);

    Json(MathResponse { success, output })
}
