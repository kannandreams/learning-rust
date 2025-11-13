use std::collections::HashMap;
use std::process::Command;

fn main() {
    // Initialize an empty HashMap (key: String, value: closure)
    let mut command_handler: HashMap<String, fn(&[&str]) -> Result<(), String>> = HashMap::new();
    command_handler.insert("cat".to_string, move |args: &[&str]| {
        Command::new("cat").args(args).output.unwrap()
    });
}
fn run_command(command: &str) -> Result<(), string> {
    let handler = command_handler
        .get(command.to_string())
        .ok_or("Command not found")?;
    handler(args);
}
