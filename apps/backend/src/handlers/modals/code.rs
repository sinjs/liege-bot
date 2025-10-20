use std::sync::Arc;

use anyhow::anyhow;
use serenity::all::{
    ActionRowComponent, Color, CreateActionRow, CreateEmbed, CreateEmbedFooter, CreateInputText,
    CreateInteractionResponseFollowup, CreateModal, InputText, InputTextStyle, ModalInteraction,
};

use crate::{
    AppState,
    env::ENV,
    error::Error,
    models::{
        api::code::{ExecuteFile, ExecuteRequest, ExecuteResponse},
        custom_id::CustomId,
    },
};

use super::ModalHandler;

pub struct CodeModal;

impl ModalHandler for CodeModal {
    async fn handle_modal(
        interaction: ModalInteraction,
        state: Arc<AppState>,
    ) -> Result<(), Error> {
        let language = CustomId::try_from(interaction.data.custom_id.clone())?
            .data
            .first()
            .ok_or(anyhow!("Failed to get language from custom id"))?
            .clone();

        let ActionRowComponent::InputText(InputText {
            value: Some(code), ..
        }) = interaction
            .data
            .components
            .first()
            .ok_or(anyhow!("Failed to get code action row"))?
            .components
            .first()
            .ok_or(anyhow!("Failed to get code component"))?
        else {
            return Err(anyhow!("Failed to get code input"));
        };

        interaction.defer(&state.serenity_http).await?;

        let response = state
            .http_client
            .post("https://v2-api.nigga.church/code/execute")
            .header("Authorization", &ENV.code_token)
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

    /// `data` is required for this modal.
    fn modal(data: Option<Vec<String>>) -> CreateModal {
        CreateModal::new(
            CustomId::new("code")
                .data(data.unwrap())
                .try_to_string()
                .unwrap(),
            "Execute Code",
        )
        .components(vec![CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Paragraph, "Code", "code").required(true),
        )])
    }
}
