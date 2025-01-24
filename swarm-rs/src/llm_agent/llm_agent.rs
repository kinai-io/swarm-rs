use std::{any::Any, collections::HashMap};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    agent::{Action, Agent, Output},
    prelude::Swarm,
};

use super::{
    llm_client::LLMClient,
    llm_message::{LLMMessage, LLMMessageBuilder},
};

#[derive(Serialize, Deserialize)]
pub struct LLMAgent {
    id: String,
    client: LLMClient,
    operations: HashMap<String, LLMAgentOperation>,
}

impl LLMAgent {

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub async fn run_operation(&self, operation: &LLMAgentOperation, prompt: &str) -> Output {
        let output_format = if let Some(format) = &operation.output_format {
            Some(serde_json::to_string(format).unwrap())
        } else {
            None
        };
        let messages = operation.build_messages(prompt);
        let response = self
            .client
            .autocomplete(&messages, output_format.clone())
            .await;
        if let Ok(response) = response {
            if output_format.is_some() {
                if let Some(result) = response.get_output::<Value>() {
                    Output::new_success(result)
                } else {
                    Output::new_error("Unable to read response")
                }
            } else {
                if let Some(result) = response.get_message() {
                    Output::new_success(result)
                } else {
                    Output::new_error("Unable to read response")
                }
            }
        } else {
            println!("response : {:?}", response);
            Output::new_error("Unable to get summary")
        }
    }
    
}

#[async_trait]
impl Agent for LLMAgent {

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self, action: &Action, _swarm: &Swarm) -> Output {
        if let Some(operation) = self.operations.get(action.get_name()) {
            if let Ok(prompt) = action.get_payload::<String>() {
                let res = self.run_operation(operation, &prompt).await;
                res
            } else {
                Output::new_error("Invalid payload")
            }
        } else {
            Output::new_error("Unknown Action")
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct LLMAgentOperation {
    role: String,
    goal: String,
    output_rules: String,
    output_format: Option<Value>,
}

impl LLMAgentOperation {
    pub fn build_messages(&self, prompt: &str) -> Vec<LLMMessage> {
        let messages = LLMMessageBuilder::new()
            .add_system_message(&self.role)
            .add_system_message(&self.goal)
            .add_system_message(&self.output_rules)
            .add_user_message(prompt)
            .build();
        messages
    }
}

