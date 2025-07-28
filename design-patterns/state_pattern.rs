///////////////////////////////
// State Pattern in Rust   //
///////////////////////////////

trait State {
    fn process(self: Box<Self>, ctx: &mut Pipeline);
    fn status(&self) -> &str;
}

struct Pending;
struct Running;
struct Completed;

impl State for Pending {
    fn process(self: Box<Self>, ctx: &mut Pipeline) {
        println!("Starting..."); 
        // Transition to the Running state
      	ctx.state = Some(Box::new(Running));
    }
    fn status(&self) -> &str { "Pending" }
}

impl State for Running {
    fn process(self: Box<Self>, ctx: &mut Pipeline) {
        println!("Running..."); 
      	ctx.state = Some(Box::new(Completed));
    }
    fn status(&self) -> &str { "Running" }
}

impl State for Completed {
    fn process(self: Box<Self>, _ctx: &mut Pipeline) {
        println!("Already completed.");
    }
    fn status(&self) -> &str { "Completed" }
}

// The Pipeline context that holds the current state
// dynamic dispatch (dyn state) is used because the state types are not known at compile time
// we use a Box to store the state trait object - this allocates the state on the heap
struct Pipeline { state: Option<Box<dyn State>> }

impl Pipeline {
    // Constructor to initialize the pipeline in the Pending state
    fn new() -> Self { Self { state: Some(Box::new(Pending)) } }

    // Process the current state, which may change the state
    // This method will call the process method of the current state and may change the state
    fn process(&mut self) {
        // We move the state out temporarily with .take()
        // this allows us to replace the state without borrowing issues 
        // ✅ one mutable reference (&mut) or ✅ multiple immutable references (&) ❌ but NOT both at the same time
        if let Some(s) = self.state.take() { s.process(self); }
    }
    fn status(&self) -> &str { self.state.as_ref().unwrap().status() }
}

fn main() {
    let mut p = Pipeline::new();
    // for loop to process the pipeline 3 times
    for _ in 0..3 {
        println!("Status: {}", p.status());
        p.process();
    }
}

// Output: // 
// Status: Pending
// Starting...
// Status: Running
// Running...
// Status: Completed
// Already completed.
