use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    env::ENV,
    error::Error,
    models::{api, auth::Claims},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeRequest {
    pub language: String,
    pub code: String,
}

pub async fn post(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CodeRequest>,
) -> Response {
    match execute_code(&state.http_client, &body.language, &body.code).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "failed to execute code").into_response(),
    }
}

async fn execute_code(
    http: &reqwest::Client,
    language: &str,
    code: &str,
) -> Result<api::code::ExecuteResponse, Error> {
    let response = http
        .post("https://v2-api.nigga.church/code/execute")
        .header("Authorization", &ENV.code_token)
        .json(
            &api::code::ExecuteRequest::new()
                .language(language)
                .version("*")
                .add_file(api::code::ExecuteFile::new().content(code)),
        )
        .send()
        .await?
        .error_for_status()?
        .json::<api::code::ExecuteResponse>()
        .await?;

    Ok(response)
}
