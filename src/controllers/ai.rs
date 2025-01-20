use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{sse, IntoResponse, Response, Sse},
    Json,
};
use axum_extra::extract::Query;
use futures::{Stream, StreamExt};
use reqwest::Url;
use reqwest_eventsource::{
    retry::{self},
    RequestBuilderExt,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::{
        api::ai::{
            GenerateImageRequest, GenerateImageResponse, GenerateTextMessage,
            GenerateTextMessageRole, GenerateTextRequest, GenerateTextStreamResponse,
        },
        auth::Claims,
    },
    AppState,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AiModelType {
    Text,
    Image,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AiRequest {
    pub model_type: AiModelType,
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AiImageResponse {
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum AiEvent {
    Done,
    Response(String),
}

pub async fn get(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Query(body): Query<AiRequest>,
) -> Response {
    match body.model_type {
        AiModelType::Image => match generate_image(&state.http_client, body.prompt).await {
            Ok(url) => (
                StatusCode::OK,
                Json(AiImageResponse {
                    image_url: url.to_string(),
                }),
            )
                .into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate image",
            )
                .into_response(),
        },

        AiModelType::Text => match generate_text(&state.http_client, body.prompt).await {
            Ok(sse) => sse.into_response(),
            Err(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate text").into_response()
            }
        },
    }
}

async fn generate_text(
    http: &reqwest::Client,
    prompt: String,
) -> Result<Sse<impl Stream<Item = Result<sse::Event, Infallible>>>, Error> {
    let mut event_source = http
        .post("https://ai.nigga.church/v2/generate/text")
        .header("Authorization", std::env::var("AI_TOKEN").unwrap())
        .json(
            &GenerateTextRequest::new()
                .model("llama-3-8b-instruct")
                .add_message(GenerateTextMessage::new(
                    GenerateTextMessageRole::User,
                    &prompt,
                ))
                .stream(true),
        )
        .eventsource()?;

    event_source.set_retry_policy(Box::new(retry::Never));

    let event_stream = event_source
        .map(|event| -> Option<sse::Event> {
            match event {
                Ok(reqwest_eventsource::Event::Open) => {
                    tracing::debug!("opening sse");
                    None
                }
                Ok(reqwest_eventsource::Event::Message(message)) => {
                    if message.data == "[DONE]" {
                        return Some(
                            sse::Event::default().data(serde_json::to_string(&AiEvent::Done).ok()?),
                        );
                    }

                    let data =
                        serde_json::from_str::<GenerateTextStreamResponse>(&message.data).ok()?;

                    Some(
                        sse::Event::default()
                            .data(serde_json::to_string(&AiEvent::Response(data.response)).ok()?),
                    )
                }
                Err(error) => {
                    tracing::error!(%error, "failed to get next event of sse stream");
                    None
                }
            }
        })
        .filter_map(|event| async move { event })
        .map(|event| Ok(event));

    Ok(Sse::new(event_stream))
}

async fn generate_image(http: &reqwest::Client, prompt: String) -> Result<Url, Error> {
    let response = http
        .post("https://ai.nigga.church/v2/generate/image")
        .header("Authorization", std::env::var("AI_TOKEN").unwrap())
        .json(
            &GenerateImageRequest::new()
                .source("codename-comet")
                .prompt(prompt),
        )
        .send()
        .await?
        .error_for_status()?;

    let response = response.json::<GenerateImageResponse>().await?.image_url;

    Ok(response)
}
