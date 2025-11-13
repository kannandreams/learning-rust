### Return vs Expression Key difference

1️⃣ `return Some(file_path);`

**Behavior:**
Immediately exits the function and returns Some(file_path) to the caller. Rust stops executing the function at that point.

This is explicit return syntax.

2️⃣ `Some(file_path);` (without return)

**Behavior:**
This is just an expression evaluated and then discarded because the for loop expects statements.
Rust does not automatically return it, so the function continues executing the loop or the rest of the code.
At the end of the function, if you didn’t explicitly return something, the return value will be () (unit) or cause a type mismatch if the function expects Option<PathBuf>.


```rust
fn foo1() -> Option<i32> {
    return Some(42);   // immediately returned
}

fn foo2() -> Option<i32> {
    Some(42)            // automatically returned (last expression, no semicolon)
}

fn foo3() -> Option<i32> {
    Some(42);           // ❌ evaluates, discards; function returns ()
}
```