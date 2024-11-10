//public module declaration
pub mod database {
    // private function by default in a module and can be accessed only within the module
    fn connect() {
        println!("connected to database");
    }
    // public function
    pub fn test_connection() {
        connect();
        println!("connected to database");
    }
}

//
fn test_connection() {
    println!("not connected to database");
}

fn main() {
    // Modules allow disambiguation between items that have the same name.
    test_connection();
    database::test_connection();
}
