pub mod greet {
    pub fn greet_user(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

fn main() {
    println!("Rust Learning Package - Main process!");
    print!("Calling greet_user() from greet module: ");
    println!("{}", greet::greet_user("Rustacean"));
}
