fn main() {
    let r;
    {
        let x = 42;
        r = &x; // r borrows x
    }
    // println!("r: {}", r); // this would cause an error
}
// Error: borrow of possibly dangling reference: `x`
//      println!("r: {}", r); // this would cause an error
//|                     ^ value borrowed here after end of scope
// r is no longer valid here because x has gone out of scope

// Every { ... } block in Rust defines a scope —
// a region of code where variables exist and are valid.
// Rust checks:

// “How long does x live?” and “How long will r be used?”
// It sees that:
// x will die at the end of the inner block.
// r will still be used later (outside the block).
// So right there, at the line r = &x;, the compiler already knows this reference would outlive x.
// Hence the compiler throws the error on that line, not when you print.


// fn main() {
//     let x = 5;
//     let r = &x; // reference is valid while x is alive
//     println!("{}", r); // works fine
// }