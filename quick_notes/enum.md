### Rust Enums: What They Are

In Rust, an `Enum` (short for enumeration) is a user-defined data type that can take multiple variants. Each variant can:

1. Be a unit variant (no data, e.g., Red).
2. Have associated data (e.g., Red(u8)).

```rust
enum Message {
    // Unit variant
    Quit,
    // Variant with associated data
    Print(String),
    // Variant with multiple fields
    Hello { greeting: String, count: u32 },
}
```

**Important Points to Note When Implementing Rust Enums**

1. Exhaustive Matching:
Rust requires match statements to handle all variants (no implicit fallback). This is a compile-time safety check.

Example:

```rust
match message {
    Message::Quit => println!("Quit"),
    Message::Print(s) => println!("Print: {}", s),
    Message::Hello { greeting, count } => println!("Hello {}x", greeting),
}
```

2. Discriminant Values:
Rust `enums` implicitly have discriminant values (integers) to differentiate variants. These are not visible in code but help the compiler.

3. Unit Variants:
Variants like Quit (no fields) are called "unit variants." They’re lightweight and useful for error handling.

4. No Implicit Conversions:
Rust won’t automatically convert between enum variants (e.g., you can’t do `Message::Quit == Message::Print("a")`). Explicit match is required.

5. Use Cases:
Enums are ideal for error handling (`Result = Ok(T) or Err(E)`), state machines, and discrete data types.

6. Avoid Over-Engineering:
Don’t create overly large enums (e.g., 100+ variants). Use structs or other types for complex cases to improve readability.