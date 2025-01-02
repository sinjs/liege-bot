use std::sync::Arc;

use anyhow::anyhow;
use serenity::all::{
    Color, CommandInteraction, CommandOptionType, CreateActionRow, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateModal, InputText,
    InputTextStyle, InstallationContext, InteractionContext, ResolvedOption, ResolvedValue,
};

use crate::{
    error::Error,
    models::{
        api::code::{ExecuteFile, ExecuteRequest, ExecuteResponse},
        custom_id::CustomId,
    },
    AppState,
};

use super::Command;

pub struct CodeCommand;

impl Command for CodeCommand {
    async fn run(interaction: CommandInteraction, state: Arc<AppState>) -> Result<(), Error> {
        let options = interaction.data.options();

        let &ResolvedOption {
            value: ResolvedValue::String(language),
            ..
        } = options.get(0).ok_or(anyhow!("Failed to get language"))?
        else {
            return Err(anyhow!("Failed to get language value as string"));
        };

        let Some(&ResolvedOption {
            value: ResolvedValue::String(code),
            ..
        }) = options.get(1)
        else {
            let modal = CreateModal::new(
                CustomId::new("code")
                    .add_data(language.to_string())
                    .try_to_string()?,
                "Execute Code",
            )
            .components(vec![CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "Code", "code").required(true),
            )]);

            interaction
                .create_response(
                    &state.serenity_http,
                    CreateInteractionResponse::Modal(modal),
                )
                .await?;

            return Ok(());
        };

        interaction.defer(&state.serenity_http).await?;

        let response = state
            .http_client
            .post("https://v2-api.nigga.church/code/execute")
            .header("Authorization", std::env::var("CODE_TOKEN").unwrap())
            .json(
                &ExecuteRequest::new()
                    .language(language)
                    .version("*")
                    .add_file(ExecuteFile::new().content(code)),
            )
            .send()
            .await?
            .error_for_status()?;

        let response = response.json::<ExecuteResponse>().await?;

        if response
            .compile
            .as_ref()
            .is_some_and(|s| s.code.is_some_and(|c| c != 0))
        {
            let embed = CreateEmbed::new()
                .color(Color::RED)
                .title("Compile Error")
                .description(format!("```\n{}\n```", {
                    let compile = response.compile.as_ref().unwrap();

                    if compile.output.is_empty() {
                        format!("[Exited with code {}]", compile.code.unwrap())
                    } else {
                        compile.output.clone()
                    }
                }))
                .footer(CreateEmbedFooter::new(format!(
                    "Language: {} {}",
                    response.language, response.version
                )));

            interaction
                .create_followup(
                    &state.serenity_http,
                    CreateInteractionResponseFollowup::new().add_embed(embed),
                )
                .await?;
        } else {
            let embed = CreateEmbed::new()
                .color(Color::FOOYOO)
                .title("Output")
                .description(format!(
                    "```\n{}\n```",
                    if response.run.output.is_empty() {
                        "No output, try logging the expression."
                    } else {
                        &response.run.output
                    }
                ))
                .footer(CreateEmbedFooter::new(format!(
                    "Language: {} {}",
                    response.language, response.version
                )));

            interaction
                .create_followup(
                    &state.serenity_http,
                    CreateInteractionResponseFollowup::new().add_embed(embed),
                )
                .await?;
        }

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("code")
            .description("Execute code")
            .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
            .contexts(vec![
                InteractionContext::Guild,
                InteractionContext::BotDm,
                InteractionContext::PrivateChannel,
            ])
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "language",
                    "Programming language to use",
                )
                .required(true)
                .add_string_choice("JavaScript", "js")
                .add_string_choice("Rust", "rust")
                .add_string_choice("Python", "python"),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "code", "The code to execute")
                    .required(false),
            )
    }
}
