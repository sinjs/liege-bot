#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Language {
    pub language: String,
    pub version: String,
    pub aliases: Vec<String>,
    pub runtime: Option<String>,
}

pub type LanguagesResponse = Vec<Language>;

#[derive(Serialize, Deserialize, Default)]
#[skip_serializing_none]
pub struct ExecuteFile {
    name: Option<String>,
    content: String,
    encoding: Option<String>,
}

impl ExecuteFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn encoding(mut self, encoding: impl Into<String>) -> Self {
        self.encoding = Some(encoding.into());
        self
    }
}

#[derive(Serialize, Deserialize, Default)]
#[skip_serializing_none]
pub struct ExecuteRequest {
    language: String,
    version: String,
    files: Vec<ExecuteFile>,
    stdin: Option<String>,
    args: Option<Vec<String>>,
    run_timeout: Option<usize>,
    compile_timeout: Option<usize>,
    compile_memory_limit: Option<isize>,
    run_memory_limit: Option<isize>,
}

impl ExecuteRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn add_file(mut self, file: ExecuteFile) -> Self {
        self.files.push(file);
        self
    }

    pub fn files(mut self, files: Vec<ExecuteFile>) -> Self {
        self.files = files;
        self
    }

    pub fn stdin(mut self, stdin: impl Into<String>) -> Self {
        self.stdin = Some(stdin.into());
        self
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = Some(args);
        self
    }

    pub fn run_timeout(mut self, timeout: usize) -> Self {
        self.run_timeout = Some(timeout);
        self
    }

    pub fn compile_timeout(mut self, timeout: usize) -> Self {
        self.compile_timeout = Some(timeout);
        self
    }

    pub fn compile_memory_limit(mut self, limit: isize) -> Self {
        self.compile_memory_limit = Some(limit);
        self
    }

    pub fn run_memory_limit(mut self, limit: isize) -> Self {
        self.run_memory_limit = Some(limit);
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExecuteStage {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: Option<isize>,
    pub signal: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub language: String,
    pub version: String,
    pub run: ExecuteStage,
    pub compile: Option<ExecuteStage>,
}
