///////////////////////////////
// Singleton Pattern in Rust //
///////////////////////////////
/* 
- OnceLock is a data structure designed to be written to ( "locked in") exactly once. 
- Lazy Initialization. It is only created the very first time the get_config() function is called.
- static means CONFIG will exist for the entire duration of the program's execution
*/

use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    CONFIG.get_or_init(|| "Loaded config".to_string())
}
