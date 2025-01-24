use serde::{Deserialize, Serialize};

use swarm_rs::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RagQuery {
    text: String,
}

impl RagQuery {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

pub struct RAGDemo {}

impl RAGDemo {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Agent for RAGDemo {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self, action: &Action, agent_swarm: &Swarm) -> Output {
        let rag_query: RagQuery = action.get_payload().unwrap();

        let query = SearchQuery {
            terms: rag_query.text,
            lang: None,
        };

        // Execute market doc search
        let doc_search_output = agent_swarm.execute("searx_ng", &query).await;

        // Consolidate context
        let docs: SearxResponse = doc_search_output.get_payload();
        let contents: Vec<&str> = docs
            .results
            .iter()
            .map(|doc| doc.content.as_str())
            .collect();

        let context_text = contents.join("\n");
        let output = agent_swarm.execute("llm-summarizer.summarize", &context_text).await;
        output
    }
}

#[tokio::test]
pub async fn workflow() {
    let searx_agent: SearxAgent = json_io::load("test-data/agents/searxng.json").unwrap();
    let llm_agent: LLMAgent = json_io::load("test-data/agents/llm_summarizer.json").unwrap();

    let rag_agent = RAGDemo::new();

    let mut agent_swarm = Swarm::new();

    agent_swarm.register_agent(&searx_agent.get_id(), searx_agent);
    agent_swarm.register_agent(&llm_agent.get_id(), llm_agent);
    agent_swarm.register_agent("rag_search", rag_agent);

    let text = r#"agentic ai system"#;
    let query = RagQuery::new(text);
    let output = agent_swarm.execute("rag_search", &query).await;

    if output.is_success() {
        println!("SUCCESS : {}", output.get_value());
    } else {
        println!("ERROR : {}", output.get_error_message());
    }
}
