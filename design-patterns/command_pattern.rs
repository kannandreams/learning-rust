///////////////////////////////
// Command Pattern in Rust   //
///////////////////////////////

trait DataCommand {
    fn execute(&self);
}

struct ExtractCommand;
impl DataCommand for ExtractCommand {
    fn execute(&self) {
        println!("Extracting data from source");
    }
}

struct TransformCommand;
impl DataCommand for TransformCommand {
    fn execute(&self) {
        println!("Transforming data");
    }
}

struct LoadCommand;
impl DataCommand for LoadCommand {
    fn execute(&self) {
        println!("Loading data to warehouse");
    }
}

// The Pipeline (Invoker)
struct DataPipeline {
    steps: Vec<Box<dyn DataCommand>>,
}

impl DataPipeline {
    fn new() -> Self {
        Self { steps: vec![] }
    }

    fn add_step(&mut self, cmd: Box<dyn DataCommand>) {
        self.steps.push(cmd);
    }

    fn run(&self) {
        for step in &self.steps {
            step.execute();
        }
     }
   }
}

// main call
fn main() {
    let mut pipeline = DataPipeline::new();
    pipeline.add_step(Box::new(ExtractCommand));
    pipeline.add_step(Box::new(TransformCommand));
    pipeline.add_step(Box::new(LoadCommand));

    pipeline.run();// Execute ETL
}
