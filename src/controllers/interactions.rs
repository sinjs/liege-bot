use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
    Interaction,
};

use crate::{error::Error, handlers, AppState};

pub async fn post(headers: HeaderMap, State(state): State<Arc<AppState>>, body: Bytes) -> Response {
    if let Err(_) = verify_signature(&state, &headers, &body) {
        return (StatusCode::UNAUTHORIZED, "failed to verify signature").into_response();
    }

    let Json(interaction) = match Json::<Interaction>::from_bytes(&body) {
        Ok(json) => json,
        Err(_) => return (StatusCode::BAD_REQUEST, "failed to parse json").into_response(),
    };

    if let Interaction::Ping(_) = interaction {
        return (StatusCode::OK, Json(CreateInteractionResponse::Pong)).into_response();
    }

    tokio::spawn(async move {
        if let Err(error) = handle_interaction(interaction.clone(), state.clone()).await {
            handle_interaction_error(error, interaction, state).await;
        }
    });

    (StatusCode::ACCEPTED, "").into_response()
}

async fn handle_interaction_error(error: Error, interaction: Interaction, state: Arc<AppState>) {
    eprintln!("Failed to handle interaction: {}", error);

    let error_message = format!("Failed to handle interaction:\n```\n{}\n```", error);

    match interaction {
        Interaction::Command(interaction) => {
            if let Err(_) = interaction
                .create_response(
                    &state.serenity_http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(&error_message),
                    ),
                )
                .await
            {
                interaction
                    .edit_response(
                        &state.serenity_http,
                        EditInteractionResponse::new().content(&error_message),
                    )
                    .await
                    .ok();
            };
        }
        Interaction::Component(interaction) => {
            if let Err(_) = interaction
                .create_response(
                    &state.serenity_http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(&error_message),
                    ),
                )
                .await
            {
                interaction
                    .edit_response(
                        &state.serenity_http,
                        EditInteractionResponse::new().content(&error_message),
                    )
                    .await
                    .ok();
            };
        }
        Interaction::Modal(interaction) => {
            if let Err(_) = interaction
                .create_response(
                    &state.serenity_http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(&error_message),
                    ),
                )
                .await
            {
                interaction
                    .edit_response(
                        &state.serenity_http,
                        EditInteractionResponse::new().content(&error_message),
                    )
                    .await
                    .ok();
            };
        }

        _ => (),
    }
}

async fn handle_interaction(interaction: Interaction, state: Arc<AppState>) -> Result<(), Error> {
    match interaction {
        Interaction::Command(interaction) => {
            handlers::commands::handle_interaction(interaction, state).await?;
        }
        Interaction::Modal(interaction) => {
            handlers::modals::handle_interaction(interaction, state).await?;
        }
        _ => {}
    }

    Ok(())
}

fn verify_signature(state: &Arc<AppState>, headers: &HeaderMap, body: &Bytes) -> Result<(), Error> {
    let signature = headers
        .get("X-Signature-Ed25519")
        .ok_or_else(|| anyhow!("missing signature header"))?
        .to_str()?;
    let timestamp = headers
        .get("X-Signature-Timestamp")
        .ok_or_else(|| anyhow!("missing timestamp header"))?
        .to_str()?;

    state
        .verifier
        .verify(signature, timestamp, body)
        .map_err(|_| anyhow!("failed to verify signature"))?;

    Ok(())
}
