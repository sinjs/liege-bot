use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Runtime {
    pub language: String,
    pub version: String,
    pub aliases: Vec<String>,
    pub runtime: Option<String>,
}

pub type RuntimesResponse = Vec<Runtime>;

#[derive(Serialize, Deserialize, Default)]
#[skip_serializing_none]
pub struct ExecuteFile {
    name: Option<String>,
    content: String,
    encoding: Option<String>,
}

impl ExecuteFile {
    fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn encoding(mut self, encoding: String) -> Self {
        self.encoding = Some(encoding);
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
