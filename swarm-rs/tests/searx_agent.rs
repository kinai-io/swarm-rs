use swarm_rs::prelude::*;

#[tokio::test]
pub async fn searx_agent() {
    let searx_agent: SearxAgent = json_io::load("test-data/agents/searxng.json").unwrap();

    let mut agent_swarm = Swarm::new();
    agent_swarm.register_agent(&searx_agent.get_id(), searx_agent);

    let text = r#"what is agentic ai system"#;

    let query = SearchQuery {
        terms: text.to_string(),
        lang: None,
    };

    let output = agent_swarm.execute("searx_ng", &query).await;

    if output.is_success() {
        println!("SUCCESS : {}", output.get_value());
    } else {
        println!("ERROR : {}", output.get_error_message());
    }
}