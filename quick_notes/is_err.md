Here’s what the function returns under the hood:

```rust
fn read_line(&mut self, buf: &mut String) -> Result<usize, std::io::Error>
```

general pattern is

```rust
match io::stdin().read_line(&mut input) {
    Ok(_) => { /* all good */ }
    Err(e) => {
        eprintln!("Failed to read input: {e}");
        continue;
    }
}
```

You can see `.is_err()` is a shorthand for that — it skips the matching and only checks the error condition.

```rust
if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }
```

or 

```rust
if let Err(e) = io::stdin().read_line(&mut input) {
    eprintln!("Failed to read input: {e}");
    continue;
}
```

`is_err` is better because:

- A read error (like Ctrl+D or bad stream) shouldn’t kill the shell.

- The loop continues safely.

- It matches the philosophy of a resilient command interpreter.


Note: Avoid using in production code

```rust
io::stdin().read_line(&mut input).unwrap();
```

`.unwrap()` means:

If it’s Ok, give me the value.
If it’s Err, panic immediately and crash the program.

