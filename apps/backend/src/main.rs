use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use env::ENV;
use error::Error;
use handlers::commands::CommandHandler;
use middleware::ratelimit::JwtKeyExtractor;
use reqwest::Client;
use serenity::all::{
    ApplicationId, CommandType, CreateCommand, EntryPointHandlerType, GuildId, InstallationContext,
    InteractionContext,
};
use serenity::interactions_endpoint::Verifier;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod args;
mod controllers;
mod env;
mod error;
mod handlers;
mod math;
mod middleware;
mod models;

pub struct AppState {
    verifier: Verifier,
    http_client: reqwest::Client,
    serenity_http: serenity::http::Http,
}

impl Default for AppState {
    fn default() -> Self {
        let state = Self {
            verifier: Verifier::new(&ENV.discord_public_key),
            http_client: Client::default(),
            serenity_http: serenity::http::Http::new(&ENV.discord_token),
        };

        state.serenity_http.set_application_id(
            ApplicationId::from_str(&ENV.discord_app_id).expect("Invalid Application ID"),
        );

        state
    }
}

async fn run() -> Result<(), Error> {
    let state = Arc::new(AppState::default());

    let api_governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(8)
            .key_extractor(JwtKeyExtractor)
            .finish()
            .unwrap(),
    );
    let auth_governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(4)
            .burst_size(2)
            .key_extractor(JwtKeyExtractor)
            .finish()
            .unwrap(),
    );

    let api_router = Router::new()
        .route("/ai", get(controllers::ai::get))
        .route("/code", post(controllers::code::post))
        .route("/math", post(controllers::math::post))
        .layer(GovernorLayer {
            config: api_governor_config,
        });

    let auth_router = Router::new()
        .route(
            "/token",
            get(controllers::token::get).post(controllers::token::post),
        )
        .layer(GovernorLayer {
            config: auth_governor_config,
        });

    let app = Router::new()
        .route("/config", get(controllers::config::get))
        .route("/interactions", post(controllers::interactions::post))
        .merge(api_router)
        .merge(auth_router)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8787").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    tracing::debug!("Listening on http://0.0.0.0:8787");

    Ok(())
}

async fn register_commands(guild_id: Option<String>) -> Result<(), Error> {
    let http = serenity::http::Http::new(&ENV.discord_token);

    http.set_application_id(ApplicationId::from_str(&ENV.discord_app_id)?);

    {
        let global_commands = http.get_global_commands().await?;
        let entry_point_id = global_commands
            .iter()
            .find(|c| c.kind == CommandType::PrimaryEntryPoint)
            .map(|c| c.id);

        if let Some(entry_point_id) = entry_point_id {
            serenity::all::Command::delete_global_command(&http, entry_point_id).await?;
        }
    }

    let entry_point_command = CreateCommand::new("launch")
        .kind(CommandType::PrimaryEntryPoint)
        .description("Launch the Liege activity")
        .handler(EntryPointHandlerType::DiscordLaunchActivity)
        .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::BotDm,
            InteractionContext::PrivateChannel,
        ]);

    let commands = vec![
        entry_point_command,
        handlers::commands::MathCommand::command(),
        handlers::commands::CodeCommand::command(),
        handlers::commands::AiCommand::command(),
    ];

    match guild_id {
        Some(guild_id) => {
            let guild_id = GuildId::from_str(&guild_id)?;
            println!(
                "Registering {} commands for guild {}",
                commands.len(),
                guild_id
            );
            guild_id.set_commands(&http, commands).await?;
        }
        None => {
            println!("Registering {} global commands", commands.len());
            serenity::all::Command::set_global_commands(&http, commands).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = args::Cli::parse();

    match args.command() {
        args::Command::Run => run().await,
        args::Command::RegisterCommands { guild_id } => register_commands(guild_id).await,
    }
}
