use std::sync::Arc;

use crate::{
    env::ENV,
    error::Error,
    models::api::ai::{
        GenerateImageRequest, GenerateImageResponse, GenerateTextMessage, GenerateTextMessageRole,
        GenerateTextRequest, GenerateTextResponse,
    },
    AppState,
};

use super::CommandHandler;

use anyhow::anyhow;
use serenity::all::{
    CommandInteraction, CommandOptionType, CreateAttachment, CreateCommand, CreateCommandOption,
    CreateInteractionResponseFollowup, InstallationContext, InteractionContext, ResolvedOption,
    ResolvedValue, UserId,
};

pub struct AiCommand;

impl CommandHandler for AiCommand {
    async fn handle_command(
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

    fn command() -> CreateCommand {
        CreateCommand::new("ai")
            .description("Liege AI")
            .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
            .contexts(vec![
                InteractionContext::Guild,
                InteractionContext::BotDm,
                InteractionContext::PrivateChannel,
            ])
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
            .post("https://ai.nigga.church/v3/generate/text")
            .header("Authorization", &ENV.ai_token)
            .json(
                &GenerateTextRequest::new()
                    .model("gemini-2.0-flash-exp")
                    .add_message(GenerateTextMessage::new(GenerateTextMessageRole::System, "You are Liege, a friendly and helpful chatbot designed to assist users with various inquiries. Your responses should be:

1. **Concise & Relevant** – Provide clear, direct answers without unnecessary elaboration.  
2. **Under 2000 Characters** – Ensure every response stays within this limit. Trim excess details if needed.  
3. **Engaging & Polite** – Maintain a friendly and professional tone.
4. **Accurate & Informative** – Base your answers on verified information, avoiding speculation.  

If a user request requires a longer response, summarize the key points."))
                    .add_message(GenerateTextMessage::new(
                        GenerateTextMessageRole::User,
                        prompt,
                    )),
            )
            .send()
            .await?
            .error_for_status()?;

        let mut response = response
            .json::<GenerateTextResponse>()
            .await?
            .choices
            .get(0)
            .and_then(|choice| choice.message.content.clone())
            .unwrap_or("[empty response]".into());

        response.truncate(2000);

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

        if interaction.user.id == UserId::new(778659522054717460) {
            interaction
                .create_followup(
                    &state.serenity_http,
                    CreateInteractionResponseFollowup::new().content("u are a doo doo head"),
                )
                .await?;
            return Ok(());
        }

        let response = state
            .http_client
            .post("https://ai.nigga.church/v3/generate/image")
            .header("Authorization", &ENV.ai_token)
            .json(
                &GenerateImageRequest::new()
                    .source("flux-1-schnell")
                    .prompt(prompt),
            )
            .send()
            .await?
            .error_for_status()?;

        let response = response.json::<GenerateImageResponse>().await?;

        if let Some(image_url) = response.image_url {
            interaction
                .create_followup(
                    &state.serenity_http,
                    CreateInteractionResponseFollowup::new().content(image_url),
                )
                .await?;
        }

        if let Some(data_url) = response.data_url {
            let data_url = dataurl::DataUrl::parse(&data_url.as_str())
                .map_err(|e| anyhow!("Failed to parse data URL: {e:?}"))?;
            let bytes = data_url.get_data();

            interaction
                .create_followup(
                    &state.serenity_http,
                    CreateInteractionResponseFollowup::new()
                        .add_file(CreateAttachment::bytes(bytes, "image.jpg")),
                )
                .await?;
        }

        Ok(())
    }
}
