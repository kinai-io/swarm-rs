use std::collections::HashMap;

use serde::Serialize;

use crate::agent::{Action, Agent, Output};


pub struct Swarm {
    agents: HashMap<String, Box<dyn Agent>>,
}

impl Swarm {

    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register_agent<T: Agent + 'static>(&mut self, agent_id: &str, agent: T) {
        self.agents.insert(agent_id.to_string(), Box::new(agent));
    }

    pub async fn execute<T: Serialize>(&self, action_id: &str, payload: &T) -> Output {
        let action = Action::new(action_id, payload);
        self.execute_action(&action).await
    }

    pub async fn execute_action(&self, action: &Action) -> Output {
        let agent_id = action.get_agent();
        if let Some(agent) = self.agents.get(agent_id) {
            // println!("Agent Found downcasting");
            // let agent:Option<Box<&dyn Agent>> = agent.downcast_ref();
            // if let Some(agent) = agent {
                let mut output = agent.execute(action, &self).await;
                output.agent_id = agent_id.to_string();
                return output
            // }
        }
        Output::new_error("Agent Not Found")
    }

    pub fn get_agent<T: Agent + 'static>(&self, agent_id: &str) -> Option<&T>{
        if let Some(agent) = self.agents.get(agent_id) {
            let agent = agent.as_any().downcast_ref::<T>();
            if let Some(agent) = agent {
                return Some(agent)
            }
        }
        None
    }
}
