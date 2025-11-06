
// Example demonstrating function definitions and return values in Rust
// Summary:

// return expr; always returns expr, semicolon required.
// expr (without return) returns the value only if there is no semicolon.
// expr; with semicolon returns () (unit type).

fn main() {
    let result = add(5, 10);
    println!("The sum is: {}", result);
    let difference = subtract(10, 5);
    println!("The difference is: {}", difference);
    let product = multiply(4, 3);
    println!("The product is: {}", product);
    let squared = square(6);
    println!("The square is: {}", squared);
    do_nothing(); // This function returns ()
}

fn add(a: i32, b: i32) -> i32 {
    a + b // The absence of a semicolon indicates the expression's value is the return value.
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b; // Using 'return' keyword explicitly.
}

// If you add a semicolon to the end of a function's last expression, it makes the function return ()
fn do_nothing() {
    let _x = 5; // This function returns the unit type '()'
}

fn multiply(a: i32, b: i32) -> i32 {
    let product = a * b; // Local variable
    product // Returning the local variable
}

fn square(x: i32) -> i32 {
    // x * x; // This will cause a compile-time error because of the semicolon
    x * x
}
