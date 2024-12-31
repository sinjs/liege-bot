use std::env;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use commands::Command;
use error::Error;
use reqwest::Client;
use serenity::all::ApplicationId;
use serenity::builder::*;
use serenity::interactions_endpoint::Verifier;
use serenity::json;
use serenity::model::application::*;
use tokio::sync::Mutex;

mod commands;
mod error;
mod models;

pub struct AppState {
    verifier: Verifier,
    http_client: reqwest::Client,
    serenity_http: serenity::http::Http,
}

impl Default for AppState {
    fn default() -> Self {
        let state = Self {
            verifier: Verifier::new(
                &env::var("DISCORD_PUBLIC_KEY")
                    .expect("Missing DISCORD_PUBLIC_KEY environment variable"),
            ),
            http_client: Client::default(),
            serenity_http: serenity::http::Http::new(
                &env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable"),
            ),
        };

        state.serenity_http.set_application_id(
            ApplicationId::from_str(
                &env::var("DISCORD_APP_ID").expect("Missing DISCORD_APP_ID environment variable"),
            )
            .expect("Invalid DISCORD_APP_ID environment variable"),
        );

        state
    }
}

async fn handle_command(
    interaction: CommandInteraction,
    state: Arc<AppState>,
) -> Result<(), error::Error> {
    let response = match interaction.data.name.as_str() {
        "math" => commands::MathCommand::run(interaction.clone(), state.clone()).await,
        "ai" => commands::AiCommand::run(interaction.clone(), state.clone()).await,
        name => Err(anyhow!("Command with name '{}' not found", name)),
    };

    if let Err(error) = response {
        eprintln!("Failed to handle command: {}", error);

        let error_message = format!("Failed to execute the command:\n```\n{}\n```", error);

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

    Ok(())
}

async fn handle_request(
    mut request: tiny_http::Request,
    state: Arc<AppState>,
) -> Result<(), Error> {
    let mut body = Vec::new();

    println!("Received request from {:?}", request.remote_addr());

    // Read the request body (containing the interaction JSON)
    request.as_reader().read_to_end(&mut body)?;

    {
        // Reject request if it fails cryptographic verification
        let find_header = |name| {
            Some(
                request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv(name))?
                    .value
                    .as_str(),
            )
        };
        let signature =
            find_header("X-Signature-Ed25519").ok_or(anyhow!("missing signature header"))?;
        let timestamp =
            find_header("X-Signature-Timestamp").ok_or(anyhow!("missing timestamp header"))?;
        if state.verifier.verify(signature, timestamp, &body).is_err() {
            request.respond(tiny_http::Response::empty(401))?;
            return Ok(());
        }
    }

    // Build Discord response
    let response = match json::from_slice::<Interaction>(&body)? {
        // Discord rejects the interaction endpoints URL if pings are not acknowledged
        Interaction::Ping(_) => {
            tiny_http::Response::from_data(json::to_vec(&CreateInteractionResponse::Pong)?)
                .with_header(
                    "Content-Type: application/json"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                )
        }

        Interaction::Command(interaction) => {
            handle_command(interaction, state).await?;
            tiny_http::Response::from_data(vec![]).with_status_code(202)
        }
        _ => return Ok(()),
    };

    request.respond(response)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let state = Arc::new(AppState::default());

    // Setup an HTTP server and listen for incoming interaction requests
    let server = Arc::new(
        tiny_http::Server::http("0.0.0.0:8787")
            .map_err(|error| anyhow!("failed to create server: {}", error))?,
    );

    loop {
        let server = server.clone();
        let state = state.clone();

        if let Ok(request) = server.recv() {
            println!("{:?}", &request);

            tokio::spawn(async move {
                if let Err(error) = handle_request(request, state).await {
                    eprintln!("Failed to handle request: {}", error);
                }
            });
        }
    }
}
