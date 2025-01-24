use std::any::Any;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::Swarm;

#[async_trait]
pub trait Agent: Any + Send + Sync {
    async fn execute(&self, input: &Action, swarm: &Swarm) -> Output;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    id: String,
    payload: Value,
}

impl Action {
    pub fn new<T: Serialize>(id: &str, payload: T) -> Self {
        Self {
            id: id.to_string(),
            payload: serde_json::to_value(payload).unwrap(),
        }
    }

    pub fn get_payload<T: DeserializeOwned>(&self) -> Result<T, String> {
        if let Ok(payload) = serde_json::from_value(self.payload.clone()) {
            Ok(payload)
        } else {
            Err("Unable to deserialize payload".to_string())
        }
    }

    pub fn get_name(&self) -> &str {
        if let Some((_, action_id)) = &self.id.split_once(".") {
            *action_id
        } else {
            "default"
        }
    }

    pub fn get_agent(&self) -> &str {
        if let Some((agent_id, _)) = &self.id.split_once(".") {
            *agent_id
        } else {
            &self.id
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub agent_id: String,
    status: String,
    payload: Value,
}

impl Output {
    pub fn new_success<T: Serialize>(payload: T) -> Self {
        Self {
            agent_id: "".to_string(),
            status: "SUCCESS".to_string(),
            payload: serde_json::to_value(payload).unwrap(),
        }
    }

    pub fn new_error(message: &str) -> Self {
        Self {
            agent_id: "".to_string(),
            status: "ERROR".to_string(),
            payload: serde_json::to_value(message).unwrap(),
        }
    }

    pub fn get_payload<T: DeserializeOwned>(&self) -> T {
        serde_json::from_value(self.payload.clone()).unwrap()
    }

    pub fn get_value(&self) -> &Value {
        &self.payload
    }

    pub fn get_error_message(&self) -> String {
        self.get_payload::<String>()
    }

    pub fn is_success(&self) -> bool {
        self.status.as_str() == "SUCCESS"
    }
}
