use std::collections::HashMap;

use chrono::Utc;
use serde::Serialize;

use crate::{
    agent::{Action, Agent, Output},
    logger::Logger,
};

pub struct Swarm {
    agents: HashMap<String, Box<dyn Agent>>,
    logging: Logger,
}

impl Swarm {
    pub fn default() -> Self {
        Self {
            agents: HashMap::new(),
            logging: Logger::new("logs", "swarm"),
        }
    }

    pub fn new(logs_base_dir: &str) -> Self {
        Self {
            agents: HashMap::new(),
            logging: Logger::new(logs_base_dir, "swarm"),
        }
    }

    pub fn register_agent<T: Agent + 'static>(&mut self, agent_id: &str, agent: T) {
        self.agents.insert(agent_id.to_string(), Box::new(agent));
    }

    pub async fn execute<T: Serialize>(&self, action_id: &str, payload: &T) -> Output {
        let action_ts = Utc::now().timestamp_millis() as u64;
        let event_type = format!("Action[{}:{}]", action_id, action_ts);
        self.logging.info(&event_type, payload);
        let action = Action::new(action_id, payload);
        let output = self.execute_action(&action).await;
        self.logging.info(&event_type, &output);
        output
    }

    pub async fn execute_action(&self, action: &Action) -> Output {
        let agent_id = action.get_agent();
        if let Some(agent) = self.agents.get(agent_id) {
            // println!("Agent Found downcasting");
            // let agent:Option<Box<&dyn Agent>> = agent.downcast_ref();
            // if let Some(agent) = agent {
            let mut output = agent.execute(action, &self).await;
            output.agent_id = agent_id.to_string();
            return output;
            // }
        }
        Output::new_error("Agent Not Found")
    }

    pub fn get_agent<T: Agent + 'static>(&self, agent_id: &str) -> Option<&T> {
        if let Some(agent) = self.agents.get(agent_id) {
            let agent = agent.as_any().downcast_ref::<T>();
            if let Some(agent) = agent {
                return Some(agent);
            }
        }
        None
    }
}
