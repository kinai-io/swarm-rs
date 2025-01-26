use swarm_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MacroAgent {}

#[agent]
impl MacroAgent {
    pub fn new() -> Self {
        Self {}
    }

    #[agent_action]
    pub async fn print_hello(&self, name: String) -> Result<String, String> {
        let message = format!("Hello {}", name);
        println!("{}", message);
        Ok(message)
    }

    #[agent_workflow]
    pub async fn test_workflow(&self, name: String, _swarm: &Swarm) -> Result<String, String> {
        let message = format!("Workflow invocation {}", name);
        println!("{}", message);
        Ok(message)
    }
}

#[tokio::test]
pub async fn macro_agent() {
    let test_macro_agent = MacroAgent::new();
    let mut agent_swarm = Swarm::default();

    agent_swarm.register_agent("TestMacro", test_macro_agent);

    execute_action(&agent_swarm, "TestMacro.print_hello", "User".to_string()).await;    
    execute_action(&agent_swarm, "TestMacro.test_workflow", "User".to_string()).await;
}

async fn execute_action<T: Serialize>(agent_swarm: &Swarm, action_id: &str, payload: T) {
    let output = agent_swarm.execute(action_id, &payload).await;
    if output.is_success() {
        println!("SUCCESS : {}", output.get_value());
    } else {
        println!("ERROR : {}", output.get_error_message());
    }
}