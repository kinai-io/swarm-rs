use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::llm_message::LLMMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    pub created: u32,
    pub choices: Vec<LLMChoice>,
    pub model: String,
    pub usage: LLMUsage,
    pub system_fingerprint: String,
    pub object: String,
}

impl LLMResponse {
    pub fn get_message(&self) -> Option<String> {
        if let Some(first_choice) = self.choices.first() {
            Some(first_choice.message.content.to_string())
        } else {
            None
        }
    }

    pub fn get_output<T: DeserializeOwned>(& self) -> Option<T> {
        if let Some(output_str) = self.get_message() {
            let queries = serde_json::from_str::<T>(&output_str);
            if let Ok(queries) = queries {
                return Some(queries);
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMUsage {
    pub completion_tokens: usize,
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMChoice {
    pub finish_reason: String,
    pub index: usize,
    pub message: LLMMessage,
}