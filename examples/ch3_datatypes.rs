use std::io::Bytes;

fn main() {
    //integers
    // release build - choose compliment wrapping
    // debug build - choose panic
    let f: u8 = 255; 

    // default is i32
    let x = 5;

    // default is f64
    let y = 2.0;

    // boolean
    let t = true;

    // character
    let c = 'c';

    // compound types
    // tuples
    let tup: (i32, f64, u8, &str) = (500_000, 6.4, 1, "test");

    // destructuring
    let (a, b, c, d) = tup;

    // arrays
    // in rust, arrays have fixed length
    // if you want a collection that can grow or shrink, use a vector
    let arr = [1, 2, 3, 4, 5]; 
    let first = arr[0];
    let byte = [0;8]; // create 8 value array and set to [0, 0, 0, 0, 0, 0, 0, 0]


}