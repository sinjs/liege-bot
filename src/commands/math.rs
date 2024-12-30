use anyhow::anyhow;
use serenity::all::{
    Color, CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};

use crate::error::Error;

use super::Command;

pub struct MathCommand;

impl Command for MathCommand {
    async fn run(options: &[ResolvedOption<'_>]) -> Result<CreateInteractionResponse, Error> {
        let &ResolvedOption {
            value: ResolvedValue::String(expression),
            ..
        } = options.first().ok_or(anyhow!("Failed to get expression"))?
        else {
            return Err(anyhow!("Failed to get expression value as string"));
        };

        let result = meval::eval_str(expression);
        let is_ok = result.is_ok();

        let content = format!(
            "**Expression:**\n```\n{}\n```\n**{}**:\n```{}\n```",
            expression,
            if is_ok { "Result" } else { "Error" },
            if is_ok {
                result.unwrap().to_string()
            } else {
                result.unwrap_err().to_string()
            }
        );

        let color = if is_ok { Color::FOOYOO } else { Color::RED };

        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(CreateEmbed::new().color(color).description(content)),
        ))
    }

    fn register() -> CreateCommand {
        CreateCommand::new("math")
            .description("Evaluate a math expression")
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "expression",
                "The expression to evaluate",
            ))
    }
}
