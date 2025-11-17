### What is a HashMap in Rust ?
A `HashMap` is a **dictionary-like data structure** that stores key-value pairs.
It’s like a real dictionary:

`Key`: A unique identifier (e.g., "cat")
`Value`: The data associated with that key (e.g., a function to run cat)

Rust’s `HashMap` is in the `std::collections` crate (part of the standard library — you don’t need to add it).


```rust
use std::collections::HashMap; // We'll use this

let mut command_handler = std::collections::HashMap::new();
command_handler.insert("cat", |args: &[&str]| {
    Command::new("cat").args(args).output().unwrap()
});
```

Here, command_handler is a HashMap where:
- Key : "cat" 
- Value: A closure (a function) that runs cat with user arguments

### Step-by-Step: How to Use HashMap in Example Shell use case

