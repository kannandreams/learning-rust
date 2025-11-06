fn main() {
    // Not all data types in Rust follow the ownership rules in the same way.
    // Integer types implement the Copy trait, meaning their values are copied
    // rather than moved when assigned to another variable.
    let x = 5;
    let y = x; // x is copied into y
    println!("x: {}, y: {}", x, y); // both x and y are accessible
}
