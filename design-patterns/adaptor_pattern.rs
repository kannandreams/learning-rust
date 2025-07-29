///////////////////////////////
// Factory Pattern in Rust   //
///////////////////////////////

use serde_json::{json, Value};

// 1. Legacy model
struct LegacySentimentModel;

impl LegacySentimentModel {
    fn predict(&self, text: &str) -> f32 {
        0.85 // dummy value output
    }
}

// 2. Target trait expected by the MCP system
trait MCPModel {
    fn run(&self, context: &Value) -> Value;
}

// 3. Adapter
struct SentimentAdapter {
    legacy_model: LegacySentimentModel,
}

impl SentimentAdapter {
    fn new() -> Self {
        Self {
            legacy_model: LegacySentimentModel,
        }
    }
}

impl MCPModel for SentimentAdapter {
    fn run(&self, context: &Value) -> Value {
        let input_text = context["input"].as_str().unwrap_or("");
        let score = self.legacy_model.predict(input_text);
        json!({ "sentiment_score": score })
    }
}

// 4. MCP agent runner
fn run_agent(model: &dyn MCPModel, context: &Value) {
    let output = model.run(context);
    println!("Agent Output: {}", output);
}

// 5. Example usage
fn main() {
    let model = SentimentAdapter::new();
    let ctx = json!({ "input": "When Engineers meet AI newsletter is amazing!" });
    run_agent(&model, &ctx);
}

// Notes

/*

                    +------------------------+
                    |   MCP Agent System     |
                    |  (expects MCPModel)    |
                    +------------------------+
                               |
                               | calls
                               v
                    +------------------------+
                    |     Adapter (MCPModel) |
                    |  --------------------  |
                    |  + legacy_model: T     |
                    |  + run(context): JSON  | <---- conforms to MCP trait
                    +------------------------+
                               |
                               | delegates to
                               v
                    +----------------------------+
                    |  LegacySentimentModel      |
                    |  ------------------------  |
                    |  + predict(text): float    |
                    +----------------------------+


*/