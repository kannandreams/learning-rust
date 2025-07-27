///////////////////////////////
// Factory Pattern in Rust   //
///////////////////////////////

// The common interface for all agent tools
trait Tool {
    fn execute(&self, input: &str) -> String;
}

struct WebSearchTool;
impl Tool for WebSearchTool {
    fn execute(&self, query: &str) -> String {
        format!("Searching the web for: '{}'...", query)
    }
}

struct CalculatorTool;
impl Tool for CalculatorTool {
    fn execute(&self, expr: &str) -> String {
        // In a real app, this would parse and compute.
        format!("Calculating '{}'...", expr)
    }
}

// The factory returns an Option in case the tool doesn't exist.
fn tool_factory(name: &str) -> Option<Box<dyn Tool>> {
    match name {
        "search" => Some(Box::new(WebSearchTool)),
        "calculator" => Some(Box::new(CalculatorTool)),
        _ => None, // This tool is not available
    }
}

fn main() {
    let tool_requests = vec!["search", "calculator", "file_reader"];

    // Use the factory to create the available tools
    let tools: Vec<Box<dyn Tool>> = tool_requests
        .iter()
        .filter_map(|name| tool_factory(name)) // filter_map handles the Option
        .collect();

    // The agent can now use any available tool via the same interface
    for tool in tools {
        println!("{}", tool.execute("some input"));
    }
}