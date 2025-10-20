use std::sync::{Arc, Mutex};

use codespan_reporting::term::termcolor::WriteColor;
use numbat::{NumbatError, buffered_writer::BufferedWriter, markup::Formatter};

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

pub struct BufferWriter {
    buffer: Vec<u8>,
}

impl BufferWriter {
    pub fn new() -> Self {
        BufferWriter { buffer: vec![] }
    }
}

impl BufferedWriter for BufferWriter {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.buffer).into()
    }
}

impl std::io::Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl WriteColor for BufferWriter {
    fn supports_color(&self) -> bool {
        false
    }

    fn set_color(
        &mut self,
        _spec: &codespan_reporting::term::termcolor::ColorSpec,
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn reset(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait EvaluateToString {
    fn evaluate_to_string(&mut self, input: &str, html: bool) -> Result<String, String>;
}

impl EvaluateToString for numbat::Context {
    fn evaluate_to_string(&mut self, input: &str, html: bool) -> Result<String, String> {
        let mut output = String::new();

        let mut push_diagnostics =
            |this: &Self, error: Box<dyn numbat::diagnostic::ErrorDiagnostic>| {
                let config = codespan_reporting::term::Config::default();
                let mut buffer: Box<dyn BufferedWriter> = if html {
                    Box::new(numbat::html_formatter::HtmlWriter::new())
                } else {
                    Box::new(BufferWriter::new())
                };

                for diagnostic in error.diagnostics() {
                    codespan_reporting::term::emit(
                        &mut buffer,
                        &config,
                        &this.resolver().files,
                        &diagnostic,
                    )
                    .unwrap();
                }

                let formatted = buffer.to_string();
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
                    if html {
                        let fmt = numbat::html_formatter::HtmlFormatter;
                        let s = fmt.format(&s, true);

                        push_formatln!(output, "{}", s);
                    } else {
                        push_formatln!(output, "{}", s);
                    }
                }

                let registry = self.dimension_registry();
                let result_markup =
                    interpreter_result.to_markup(statements.last(), registry, true, html);

                if html {
                    let fmt = numbat::html_formatter::HtmlFormatter;
                    let s = fmt.format(&result_markup, true);

                    push_format!(output, "{}", s);
                } else {
                    push_format!(output, "{}", result_markup);
                }
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

        if is_ok { Ok(output) } else { Err(output) }
    }
}

pub fn evaluate(input: &str) -> Result<String, String> {
    let mut context =
        numbat::Context::new(numbat::module_importer::BuiltinModuleImporter::default());

    let _ = context
        .interpret("use prelude", numbat::resolver::CodeSource::Internal)
        .unwrap();

    context.evaluate_to_string(input, false)
}

pub fn evaluate_html(input: &str) -> Result<String, String> {
    let mut context =
        numbat::Context::new(numbat::module_importer::BuiltinModuleImporter::default());

    let _ = context
        .interpret("use prelude", numbat::resolver::CodeSource::Internal)
        .unwrap();

    context.evaluate_to_string(input, true)
}
