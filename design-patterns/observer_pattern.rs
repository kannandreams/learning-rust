///////////////////////////////
// Observer Pattern in Rust   //
///////////////////////////////

// Observer Trait
trait Observer {
    fn update(&self, status: &str);
}

// Subject
struct AIAgent {
    observers: Vec<Box<dyn Observer>>,
}

impl AIAgent {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn set_status(&self, status: &str) {
        // Notify all observers
        for observer in &self.observers {
            observer.update(status);
        }
    }
}

// Concrete Observer
struct Monitor(String); // Give it a name

impl Observer for Monitor {
    fn update(&self, status: &str) {
        println!("[{}] Notified: Agent status is now '{}'", self.0, status);
    }
}

// --- Demo ---
let mut agent = AIAgent { observers: vec![] };

agent.attach(Box::new(Monitor("DashboardUI".to_string())));
agent.attach(Box::new(Monitor("AlertingService".to_string())));

agent.set_status("Processing Task...");

// Output:
// [DashboardUI] Notified: Agent status is now 'Processing Task...'
// [AlertingService] Notified: Agent status is now 'Processing Task...'