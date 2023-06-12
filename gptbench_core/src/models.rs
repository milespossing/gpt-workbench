// https://platform.openai.com/docs/api-reference/chat

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ChatMessage {
    pub fn system_message(content: String) -> Self {
        Self {
            role: ChatRole::System,
            content,
            name: None,
        }
    }
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

impl ChatRequest {
    pub fn push_user_message(mut self, content: String) -> Self {
        self.messages.push(ChatMessage {
            role: ChatRole::User,
            content,
            name: None,
        });
        self
    }
}

#[derive(Deserialize, Serialize)]
pub struct ChatResponseEntry {
    pub message: ChatMessage,
}

#[derive(Deserialize, Serialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatResponseEntry>,
}
