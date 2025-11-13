### What is String?

`String` is an owned, growable, heap-allocated string.
It actually stores the text data in memory (on the heap).
You can modify it.
You’re responsible for freeing it when it goes out of scope.

```rust
let mut name = String::from("Rust");
name.push_str(" c");  // ✅ you can modify it
println!("{}", name);
```

```bash
String ─┐
        └──> ["R", "u", "s", "t", "c"]
```

### What is &str?

`&str` is a borrowed view — a reference to an existing string slice.
- It doesn’t own the memory.
- It just points to some text that exists elsewhere.
- It’s usually read-only.

```rust
let name = String::from("Kannan");
let slice: &str = &name; // borrow a view of the string
println!("{}", slice);   // ✅ read allowed
// slice.push_str(" K"); // ❌ cannot modify

```

```bash
String ─┐
        └──> ["R", "u", "s", "t"]
              ↑
             &str (just a pointer + length)
```


**Think of it like this analogy:**

`String` → You own the book (you can write, erase, or keep it).
`&str` → You borrow the book temporarily (you can read it, but not edit or tear pages).