use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use axum::routing::{get, post};
use clap::Parser;
use error::Error;
use handlers::commands::CommandHandler;
use handlers::modals::ModalHandler;
use models::custom_id::CustomId;
use models::token::{DiscordTokenResponse, TokenRequest};
use reqwest::{Client, Url};
use serenity::all::{ApplicationId, GuildId};
use serenity::builder::*;
use serenity::interactions_endpoint::Verifier;
use serenity::json;
use serenity::model::application::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod args;
mod controllers;
mod error;
mod handlers;
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

async fn run() -> Result<(), Error> {
    let state = Arc::new(AppState::default());
    let app = axum::Router::new()
        .route("/token", post(controllers::token::post))
        .route("/interactions", post(controllers::interactions::post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8787").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register_commands(guild_id: Option<String>) -> Result<(), Error> {
    let http = serenity::http::Http::new(
        &env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable"),
    );

    http.set_application_id(ApplicationId::from_str(
        &env::var("DISCORD_APP_ID").expect("Missing DISCORD_APP_ID environment variable"),
    )?);

    let commands = vec![
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
