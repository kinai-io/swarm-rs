use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMMessage {
    pub role: String,
    pub content: String,
}

pub struct LLMMessageBuilder {
    messages: Vec<LLMMessage>,
}

impl LLMMessageBuilder {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
    pub fn add_system_message(mut self, message: &str) -> Self {
        let llm_message = LLMMessage {
            role: "system".to_string(),
            content: message.to_string(),
        };
        self.messages.push(llm_message);
        self
    }

    pub fn add_user_message(mut self, message: &str) -> Self {
        let llm_message = LLMMessage {
            role: "user".to_string(),
            content: message.to_string(),
        };
        self.messages.push(llm_message);
        self
    }

    pub fn build(self) -> Vec<LLMMessage> {
        self.messages
    }
}

