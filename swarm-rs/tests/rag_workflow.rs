use serde::{Deserialize, Serialize};

use swarm_rs::{llm_agent::llm_response::LLMResponse, prelude::*};

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

#[derive(Serialize, Deserialize)]
pub struct RagResponse {
    user_input: String,
    keywords: String,
    summary: String,
    references: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LLMSummary {
    summary: String,
    keywords: String,
}

pub struct RAGDemo {}

#[agent]
impl RAGDemo {

    pub fn new() -> Self {
        Self {}
    }

    #[agent_workflow]
    pub async fn search(&self, rag_query: RagQuery, swarm: &Swarm) -> Result<RagResponse, String> {
        let query = SearchQuery {
            terms: rag_query.text.to_string(),
            lang: None,
        };

        // Execute market doc search
        let doc_search_output = swarm.execute("searx_ng.search", &query).await;

        // Handle search output
        let docs: SearxResponse = doc_search_output.get_payload();
        let contents: Vec<&str> = docs
            .results
            .iter()
            .map(|doc| doc.content.as_str())
            .collect();

        // Prepare LLM Content
        let context_text = contents.join("\n");

        // Ask the LLM for summary
        let mut prompt = LLMPrompt::new();
        prompt.add_content("content", &context_text);

        let output = swarm.execute("llm-summarizer.execute", &prompt).await;

        let llm_response = output.get_payload::<LLMResponse>();
        if let Some(llm_summary) = llm_response.get_output::<LLMSummary>() {
            // Collect references
            let references = contents.iter().map(|content| content.to_string()).collect();

            // Build response
            let response = RagResponse {
                user_input: rag_query.text,
                keywords: llm_summary.keywords,
                summary: llm_summary.summary,
                references,
            };

            Ok(response)
        } else {
            Err("Unable to decode llm output".to_string())
        }
    }
}

#[tokio::test]
pub async fn rag_workflow() {
    let searx_agent: SearxAgent = json_io::load("test-data/agents/searxng.json").unwrap();
    let llm_agent: LLMAgent = json_io::load("test-data/agents/llm_summarizer.json").unwrap();

    let rag_agent = RAGDemo::new();

    let mut agent_swarm = Swarm::new();

    agent_swarm.register_agent(&searx_agent.get_id(), searx_agent);
    agent_swarm.register_agent(&llm_agent.get_id(), llm_agent);
    agent_swarm.register_agent("web_rag", rag_agent);

    let text = r#"agentic ai system"#;
    let query = RagQuery::new(text);
    let output = agent_swarm.execute("web_rag.search", &query).await;

    if output.is_success() {
        println!("SUCCESS : {}", output.get_value());
    } else {
        println!("ERROR : {}", output.get_error_message());
    }
}
