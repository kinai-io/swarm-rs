use swarm_rs::prelude::*;

#[tokio::test]
pub async fn llm_agent() {
    let llm_agent: LLMAgent = json_io::load("test-data/agents/llm_summarizer.json").unwrap();

    let mut agent_swarm = Swarm::default();
    agent_swarm.register_agent(&llm_agent.get_id(), llm_agent);

    let mut prompt = LLMPrompt::new();
    prompt.add_content("content", r#"
Agentic AI refers to artificial intelligence systems that exhibit characteristics of autonomy, goal-directed behavior, and decision-making akin to those of an independent agent. Unlike traditional AI systems, which operate based on predefined rules or direct human instructions, agentic AI can adapt, learn, and make decisions in complex environments without continuous human oversight.

Key characteristics of agentic AI include:

Autonomy: The ability to operate independently, initiating actions and adapting to changing conditions.
Goal-Directedness: Pursuit of specific objectives, either set externally by humans or internally generated by the system.
Self-Improvement: Continuous learning and optimization to improve performance and achieve goals more effectively.
Context Awareness: Understanding and interpreting its environment to make informed decisions.
Agentic AI is particularly valuable in scenarios requiring adaptability and complex decision-making, such as robotics, autonomous vehicles, personalized assistants, and strategic planning. However, its development raises important considerations around safety, ethical alignment, and accountability, as systems with high levels of autonomy may act in ways that are unpredictable or misaligned with human intentions.
"#);

    let action = Action::new("llm-summarizer.execute", prompt);
    let output = agent_swarm.execute_action(&action).await;

    if output.is_success() {
        println!("SUCCESS : {}", output.get_value());
    } else {
        println!("ERROR : {}", output.get_error_message());
    }
}
