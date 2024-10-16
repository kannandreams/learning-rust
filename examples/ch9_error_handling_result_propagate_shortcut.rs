use std::fs::File;
use std::io::{self, Read};

//implementation uses the ? operator.
// The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => eprintln!("Error reading username: {}", e),
    }
}


//this can be further shortend by chaining the calls
// The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. 

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

// standard library provides the convenient fs::read_to_string function that opens the file, 
// creates a new String, reads the contents of the file, 
// puts the contents into that String, and returns it. 

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }