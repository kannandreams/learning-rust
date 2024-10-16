use std::fs::File;
use std::io::ErrorKind;

fn main() {

    // shortcut for panic!
    let _greeting_file_result = File::open("hello.txt").unwrap();

    // shortcut for panic! with custom message
    let _greeting_file_result = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // advance method : using closures to handle errors in a more concise way 
    // closures better alternatives to match to handle Result<T, E>
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}