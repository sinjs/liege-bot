use std::sync::Arc;

use crate::{
    error::Error,
    models::api::ai::{
        GenerateImageRequest, GenerateImageResponse, GenerateTextMessage, GenerateTextMessageRole,
        GenerateTextRequest, GenerateTextResponse,
    },
    AppState,
};

use super::Command;

use anyhow::anyhow;
use reqwest::Method;
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
        CreateInteractionResponse, CreateInteractionResponseFollowup,
        CreateInteractionResponseMessage, EditInteractionResponse, ResolvedOption, ResolvedValue,
    },
    json,
};
use tokio::sync::Mutex;

pub struct AiCommand;

impl Command for AiCommand {
    async fn run(
        interaction: CommandInteraction,
        state: Arc<AppState>,
    ) -> Result<(), crate::error::Error> {
        interaction.defer(&state.serenity_http).await?;

        let options = interaction.data.options();

        let ResolvedOption {
            name,
            value: ResolvedValue::SubCommand(options),
            ..
        } = options
            .first()
            .ok_or(anyhow!("Failed to get subcommand"))
            .cloned()?
        else {
            return Err(anyhow!("Failed to get option value as subcommand"));
        };

        match name {
            "text" => AiCommand::run_text(&interaction, &options, state).await,
            "image" => AiCommand::run_image(&interaction, &options, state).await,
            name => Err(anyhow!("Invalid subcommand name {}", name)),
        }
    }

    fn register() -> CreateCommand {
        CreateCommand::new("ai")
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "text", "Chat with Liege")
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "prompt",
                            "Prompt for the AI",
                        )
                        .required(true),
                    ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "image",
                    "Generate an image",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "prompt",
                        "Prompt for generating the image",
                    )
                    .required(true),
                ),
            )
    }
}

impl AiCommand {
    async fn run_text(
        interaction: &CommandInteraction,
        subcommand_options: &Vec<ResolvedOption<'_>>,
        state: Arc<AppState>,
    ) -> Result<(), Error> {
        let &ResolvedOption {
            value: ResolvedValue::String(prompt),
            ..
        } = subcommand_options
            .first()
            .ok_or(anyhow!("Failed to get prompt"))?
        else {
            return Err(anyhow!("Failed to get prompt as string"));
        };

        let response = state
            .http_client
            .post("https://ai.nigga.church/v2/generate/text")
            .header("Authorization", std::env::var("AI_TOKEN").unwrap())
            .json(
                &GenerateTextRequest::new()
                    .model("llama-3-8b-instruct")
                    .add_message(GenerateTextMessage::new(
                        GenerateTextMessageRole::User,
                        prompt,
                    )),
            )
            .send()
            .await?
            .error_for_status()?;

        let response = response
            .json::<GenerateTextResponse>()
            .await?
            .response
            .unwrap_or("[empty response]".into());

        interaction
            .create_followup(
                &state.serenity_http,
                CreateInteractionResponseFollowup::new().content(response),
            )
            .await?;

        Ok(())
    }

    async fn run_image(
        interaction: &CommandInteraction,
        subcommand_options: &Vec<ResolvedOption<'_>>,
        state: Arc<AppState>,
    ) -> Result<(), crate::error::Error> {
        let &ResolvedOption {
            value: ResolvedValue::String(prompt),
            ..
        } = subcommand_options
            .first()
            .ok_or(anyhow!("Failed to get prompt"))?
        else {
            return Err(anyhow!("Failed to get prompt as string"));
        };

        let response = state
            .http_client
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

        interaction
            .create_followup(
                &state.serenity_http,
                CreateInteractionResponseFollowup::new().content(response),
            )
            .await?;

        Ok(())
    }
}
