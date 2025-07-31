///////////////////////////////
// Strategy Pattern in Rust //
///////////////////////////////

trait AIAgent {
    fn decide_action(&self, input: &str) -> Option<String>;
}

// Primary Strategy: LLM Agent (may fail or return None)
struct LLMAgent;
impl AIAgent for LLMAgent {
    fn decide_action(&self, input: &str) -> Option<String> {
        if input.contains("complex") {
            Some("LLM: Let me generate a AI response...".to_string())
        } else {
            println!("LLM unsure, falling back.");
            None // Simulate failure/uncertainty
        }
    }
}

// Fallback Strategy: Default Rule-based Agent
struct DefaultAgent;
impl AIAgent for DefaultAgent {
    fn decide_action(&self, input: &str) -> Option<String> {
        Some(format!("DefaultAgent: fallback response for '{}'", input))
    }
}

// Agent system with fallback
struct SmartAgent {
    primary: Box<dyn AIAgent>,
    fallback: Box<dyn AIAgent>,
}

impl SmartAgent {
    fn new(primary: Box<dyn AIAgent>, fallback: Box<dyn AIAgent>) -> Self {
        Self { primary, fallback }
    }

    fn handle(&self, input: &str) {
        let action = self
            .primary
            .decide_action(input)
            .or_else(|| self.fallback.decide_action(input));

        println!("Final Action: {}", action.unwrap_or("No action taken".to_string()));
    }
}

fn main() {
    let agent = SmartAgent::new(Box::new(LLMAgent), Box::new(DefaultAgent));

    agent.handle("complex request"); // Uses LLM
    agent.handle("simple task");     // Falls back to default
}

//Final Action: LLM: Let me generate a smart response...
//LLM unsure, falling back.
//Final Action: DefaultAgent: fallback response for 'simple task'
