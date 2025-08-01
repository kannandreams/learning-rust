///////////////////////////////
// Decorator Pattern in Rust //
///////////////////////////////

trait DataProcessor {
    fn process(&self, data: &str) -> String;
}

struct RawProcessor;
impl DataProcessor for RawProcessor {
    fn process(&self, data: &str) -> String {
        format!("Processed: {}", data)
    }
}

// Decorator: adds logging
struct LoggingProcessor<T: DataProcessor>(T);
impl<T: DataProcessor> DataProcessor for LoggingProcessor<T> {
    fn process(&self, data: &str) -> String {
        println!("Logging: {}", data);
        self.0.process(data)
    }
}

// Decorator: adds validation
struct ValidationProcessor<T: DataProcessor>(T);
impl<T: DataProcessor> DataProcessor for ValidationProcessor<T> {
    fn process(&self, data: &str) -> String {
        if data.is_empty() {
            "Invalid data!".into()
        } else {
            self.0.process(data)
        }
    }
}

fn main() {
    let processor = LoggingProcessor(ValidationProcessor(RawProcessor));
    let output = processor.process("event-123");
    println!("{}", output);
}
