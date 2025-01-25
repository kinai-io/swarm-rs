use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    agent::{Action, Agent, Output},
    prelude::*,
};

use super::{
    llm_client::LLMClient,
    llm_message::{LLMMessage, LLMMessageBuilder},
    llm_response::LLMResponse,
};

#[derive(Serialize, Deserialize)]
pub struct LLMPrompt {
    values: HashMap<String, String>,
}

impl LLMPrompt {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn add_content(&mut self, key: &str, content: &str) {
        self.values.insert(key.to_string(), content.to_string());
    }
}

#[derive(Serialize, Deserialize)]
pub struct LLMAgent {
    id: String,
    client: LLMClient,
    role: String,
    goal: String,
    output_rules: String,
    output_format: Option<Value>,
}

#[agent]
impl LLMAgent {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    #[agent_action]
    pub async fn execute(&self, prompt: LLMPrompt) -> Result<LLMResponse, String> {
        let output_format = if let Some(format) = &self.output_format {
            Some(serde_json::to_string(format).unwrap())
        } else {
            None
        };
        let messages = self.build_messages(&prompt);
        let response = self
            .client
            .autocomplete(&messages, output_format.clone())
            .await;
        if let Ok(response) = response {
            Ok(response)
        } else {
            println!("response : {:?}", response);
            Err("Unable to get summary".to_string())
        }
    }

    pub fn build_messages(&self, prompt: &LLMPrompt) -> Vec<LLMMessage> {
        let role_text = fill_template(&self.role, &prompt.values);
        let goal_text = fill_template(&self.goal, &prompt.values);
        let output_rules_text = fill_template(&self.output_rules, &prompt.values);

        let messages = LLMMessageBuilder::new()
            .add_system_message(&role_text)
            .add_system_message(&goal_text)
            .add_system_message(&output_rules_text)
            .build();
        messages
    }
}

pub fn fill_template(template: &str, values: &HashMap<String, String>) -> String {
    let mut output = template.to_string();
    for (key, value) in values {
        let pattern = format!("{{{}}}", key);
        output = output.replace(&pattern, value);
    }

    output
}
