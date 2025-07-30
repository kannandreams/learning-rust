///////////////////////////////
// Prototype Pattern in Rust //
///////////////////////////////

#[derive(Debug, Clone)]
// Above line is shorthand to automatically implement traits for a struct (or enum)
struct Agent {
    name: String,
    temperature: f32,
    model: String,
}

impl Agent {
    fn new(name: &str, temperature: f32, model: &str) -> Self {
        Self {
            name: name.to_string(),
            temperature,
            model: model.to_string(),
        }
    }

    fn respond(&self, prompt: &str) {
        println!(
            "[{}] (model={}, temp={}): Responding to '{}'",
            self.name, self.model, self.temperature, prompt
        );
    }
}

// Use the prototype to create new agents

fn main() {
    // Base agent prototype
    let base_agent = Agent::new("BaseAgent", 0.7, "gpt-neo");

    // Clone for a new agent
    let mut agent1 = base_agent.clone();
    agent1.name = "ToDoAgent".to_string();

    let mut agent2 = base_agent.clone();
    agent2.name = "NewsAgent".to_string();
    agent2.temperature = 0.9; // override just this

    agent1.respond("What are my priorities?");
    agent2.respond("Summarize today's AI news.");
}

// [ToDoAgent] (model=gpt-neo, temp=0.7): Responding to 'How’s the weather?'
// [NewsAgent] (model=gpt-neo, temp=0.9): Responding to 'Summarize this article.'
