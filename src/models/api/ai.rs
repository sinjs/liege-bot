#![allow(dead_code)]

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GenerateTextMessageRole {
    User,
    System,
    Assistant,
    Tool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTextMessage {
    role: GenerateTextMessageRole,
    content: String,
}

impl GenerateTextMessage {
    pub fn new(role: GenerateTextMessageRole, content: &str) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTextRequest {
    model: String,
    messages: Vec<GenerateTextMessage>,
    predefined_messages: Vec<String>,
    stream: bool,
}

impl GenerateTextRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn add_message(mut self, message: GenerateTextMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn messages(mut self, messages: Vec<GenerateTextMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn predefined_messages(mut self, predefined_messages: Vec<String>) -> Self {
        self.predefined_messages = predefined_messages;
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTextResponse {
    pub response: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateImageRequest {
    source: String,
    prompt: String,
}

impl GenerateImageRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateImageResponse {
    #[serde(rename = "imageURL")]
    pub image_url: Url,
}
