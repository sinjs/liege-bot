use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use numbat::NumbatError;
use serenity::all::{
    Color, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, InstallationContext,
    InteractionContext, ResolvedOption, ResolvedValue,
};

use crate::{error::Error, AppState};

use super::CommandHandler;

pub struct MathCommand;

impl CommandHandler for MathCommand {
    async fn handle_command(
        interaction: CommandInteraction,
        state: Arc<AppState>,
    ) -> Result<(), Error> {
        let options = interaction.data.options();

        let &ResolvedOption {
            value: ResolvedValue::String(expression),
            ..
        } = options.first().ok_or(anyhow!("Failed to get expression"))?
        else {
            return Err(anyhow!("Failed to get expression value as string"));
        };

        let result = evaluate(expression);
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

        interaction
            .create_response(
                &state.serenity_http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(CreateEmbed::new().color(color).description(content)),
                ),
            )
            .await
            .map_err(Error::from)?;

        println!("Passed Math");

        Ok(())
    }

    fn command() -> CreateCommand {
        CreateCommand::new("math")
            .description("Evaluate a math expression")
            .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
            .contexts(vec![
                InteractionContext::Guild,
                InteractionContext::BotDm,
                InteractionContext::PrivateChannel,
            ])
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "expression",
                "The expression to evaluate",
            ))
    }
}

macro_rules! push_format {
    ($string:ident, $($arg:tt)*) => {{
        let formatted = format!($($arg)*);
        $string.push_str(&formatted);
    }};
}

macro_rules! push_formatln {
    ($string:ident, $($arg:tt)*) => {{
        push_format!($string, $($arg)*);
        $string.push_str("\n");
    }};
}

trait EvaluateToString {
    fn evaluate_to_string(&mut self, input: &str) -> Result<String, String>;
}

impl EvaluateToString for numbat::Context {
    fn evaluate_to_string(&mut self, input: &str) -> Result<String, String> {
        let mut output = String::new();

        let mut push_diagnostics =
            |this: &Self, error: Box<dyn numbat::diagnostic::ErrorDiagnostic>| {
                let config = codespan_reporting::term::Config::default();
                let mut buffer = codespan_reporting::term::termcolor::Buffer::no_color();

                for diagnostic in error.diagnostics() {
                    codespan_reporting::term::emit(
                        &mut buffer,
                        &config,
                        &this.resolver().files,
                        &diagnostic,
                    )
                    .unwrap();
                }

                let formatted = String::from_utf8_lossy(buffer.as_slice());
                output.push_str(&formatted);
            };

        let to_be_printed: Arc<Mutex<Vec<numbat::markup::Markup>>> =
            Arc::new(Mutex::new(Vec::new()));
        let to_be_printed_cloned = to_be_printed.clone();

        let mut settings = numbat::InterpreterSettings {
            print_fn: Box::new(move |s: &numbat::markup::Markup| {
                to_be_printed_cloned.lock().unwrap().push(s.clone());
            }),
        };

        let result =
            self.interpret_with_settings(&mut settings, input, numbat::resolver::CodeSource::Text);

        let is_ok = result.is_ok();

        match result.map_err(|b| *b) {
            Ok((statements, interpreter_result)) => {
                let to_be_printed = to_be_printed.lock().unwrap();
                for s in to_be_printed.iter() {
                    push_formatln!(output, "{}", s);
                }

                let registry = self.dimension_registry();
                let result_markup =
                    interpreter_result.to_markup(statements.last(), registry, true, false);

                push_format!(output, "{}", result_markup);
            }

            Err(NumbatError::ResolverError(e)) => {
                push_diagnostics(self, Box::new(e));
            }
            Err(NumbatError::NameResolutionError(
                e @ (numbat::NameResolutionError::IdentifierClash { .. }
                | numbat::NameResolutionError::ReservedIdentifier(_)),
            )) => {
                push_diagnostics(self, Box::new(e));
            }
            Err(NumbatError::TypeCheckError(e)) => {
                push_diagnostics(self, Box::new(e));
            }
            Err(NumbatError::RuntimeError(e)) => {
                push_diagnostics(self, Box::new(e));
            }
        }

        let output = output.trim().to_owned();

        if is_ok {
            Ok(output)
        } else {
            Err(output)
        }
    }
}

fn evaluate(input: &str) -> Result<String, String> {
    let mut context =
        numbat::Context::new(numbat::module_importer::BuiltinModuleImporter::default());

    let _ = context
        .interpret("use prelude", numbat::resolver::CodeSource::Internal)
        .unwrap();

    context.evaluate_to_string(input)
}
