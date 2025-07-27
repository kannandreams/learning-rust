///////////////////////////////
// Builder Pattern in Rust   //
///////////////////////////////

// The final object
pub struct Server {
    hostname: String,
    port: u16,
}

// The builder
pub struct ServerBuilder {
    hostname: String,
    port: Option<u16>,
}

impl ServerBuilder {
    pub fn new(hostname: String) -> Self {
        Self { hostname, port: None }
    }
    
    // Chainable method
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    // Finalize the object
    pub fn build(self) -> Server {
        Server {
            hostname: self.hostname,
            port: self.port.unwrap_or(80), // Default port
        }
    }
}

// How to use it:
fn main() {
    let server = ServerBuilder::new("localhost".to_string())
        .port(8080)
        .build();

    println!("Server running on {}:{}", server.hostname, server.port);
    // Output: Server running on localhost:8080
}