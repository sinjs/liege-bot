use std::env;
use std::sync::Arc;

use anyhow::anyhow;
use commands::Command;
use error::Error;
use serenity::builder::*;
use serenity::interactions_endpoint::Verifier;
use serenity::json;
use serenity::model::application::*;

mod commands;
mod error;

fn handle_unknown_command(command_name: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("Command with name `{}` not found", command_name)),
    )
}

async fn handle_command(interaction: CommandInteraction) -> CreateInteractionResponse {
    let response = match interaction.data.name.as_str() {
        "math" => commands::math::MathCommand::run(&interaction.data.options()).await,
        command_name => Ok(handle_unknown_command(command_name)),
    };

    match response {
        Ok(response) => response,
        Err(error) => CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(format!("Failed to execute command: ```\n{}\n```", error)),
        ),
    }
}

async fn handle_request(
    mut request: tiny_http::Request,
    verifier: Arc<Verifier>,
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
        if verifier.verify(signature, timestamp, &body).is_err() {
            request.respond(tiny_http::Response::empty(401))?;
            return Ok(());
        }
    }

    // Build Discord response
    let response = match json::from_slice::<Interaction>(&body)? {
        // Discord rejects the interaction endpoints URL if pings are not acknowledged
        Interaction::Ping(_) => CreateInteractionResponse::Pong,
        Interaction::Command(interaction) => handle_command(interaction).await,
        _ => return Ok(()),
    };

    // Send the Discord response back via HTTP
    request.respond(
        tiny_http::Response::from_data(json::to_vec(&response)?).with_header(
            "Content-Type: application/json"
                .parse::<tiny_http::Header>()
                .unwrap(),
        ),
    )?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let verifier = Arc::new(Verifier::new(&env::var("DISCORD_PUBLIC_KEY")?));

    // Setup an HTTP server and listen for incoming interaction requests
    let server = Arc::new(
        tiny_http::Server::http("0.0.0.0:8787")
            .map_err(|error| anyhow!("failed to create server: {}", error))?,
    );

    loop {
        let server = server.clone();
        let verifier = verifier.clone();

        if let Ok(request) = server.recv() {
            println!("{:?}", &request);

            tokio::spawn(async move {
                if let Err(error) = handle_request(request, verifier).await {
                    eprintln!("Failed to handle request: {}", error);
                }
            });
        }
    }
}
