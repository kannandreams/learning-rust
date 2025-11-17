
# Learning Guide to become Rustacean 🦀

## 1. Getting Started

- Learning Guide for Rustaceans  
- Installing Rust (rustup)  
- Rust Toolchain (cargo, rustc, rustfmt, clippy)  
- Project Structure (`src/`, `Cargo.toml`)  

## 2. Rust Basics / Core Concepts

- Variables & Mutability  
- Constants & Static Variables  
- Shadowing  
- Primitive Data Types  
- Complex Data Types (Arrays, Tuples, Slices, Strings)  
- Functions  
- Control Flow: `if`, `match`, `loop`, `while`, `for`


## 3. Ownership, Borrowing & Memory

- Ownership  
- Borrowing (references, mutable references)  
- Move semantics  
- Copy vs Clone  
- Lifetimes  
- Memory Safety Rules  
- Stack vs Heap  
- Drop Trait and RAII Basics

## 4. Data Structures & Collections

- Structs  
- Enums  
- Tuples  
- Pattern Matching & Destructuring  
- Collections (Vec, HashMap, HashSet, BTreeMap, etc.)  
- Option & Result  
- Iterators  
- Closures  
- Implementing Custom Types (`impl`, methods, associated functions)


## 5. Modules, Crates & Packages

- Modules & Paths (`mod`, `pub`, `super`, `crate`)  
- Crates (binary vs library)  
- Packages & Workspaces  
- Re-exporting modules (`pub use`)  
- Standard Library Overview

## 6. Advanced Type System

- Generics  
- Traits  
- Trait Objects (`dyn Trait`)  
- Associated Types  
- Type Aliases  
- Advanced Lifetimes  
- PhantomData  
- Sized / Unsized Types  
- Newtype Pattern

## 7. Error Handling

- Result & Option  
- `?` operator  
- Panic & Unwinding  
- Custom Error Types (thiserror / anyhow)  
- Error Propagation Best Practices

## 8. Concurrency & Parallelism

### Threads & Sync Primitives
- Threads  
- Send & Sync  
- Arc  
- Mutex  
- RwLock  
- Condvar  
- Atomics

### Async Programming
- `async` / `await`  
- Futures  
- Executors (Tokio, async-std)  
- Channels (sync & async)  
- Select operations

## 8. Programming Paradigms

* Object-Oriented Programming Concepts
* Functional Programming Concepts
* Closures & Higher-Order Functions

## 9. Macros & Metaprogramming

- Declarative Macros (`macro_rules!`)  
- Procedural Macros  
  - Derive macros  
  - Attribute macros  
  - Function-like macros  
- Build scripts (`build.rs`)

## 9. Concurrency & Asynchronous Programming

* Threads & Concurrency
* `async` / `await`
* Channels & Message Passing
* Mutex & RwLock

## 10. File I/O, Serialization & Networking

- File I/O (std::fs, std::io)  
- Reading/Writing text & binary files  
- Serialization / Deserialization with Serde  
- JSON, YAML, TOML  
- Building CLI tools  
- Networking (TCP, UDP, HTTP)  
- REST APIs (Axum, Actix, Rocket)

## 11. Functional & OOP-Inspired Paradigms

- Closures  
- Higher-order functions  
- Iterator combinators  
- Composition over inheritance  
- Polymorphism through traits

## 12. Unsafe & Low-Level Rust

- When to use Unsafe Rust  
- Raw pointers  
- `unsafe` blocks  
- FFI with C  
- Memory layout (`repr`)  
- Inline assembly  
- Zero-cost abstractions

## 13. Tooling, Debugging & Testing

- Unit Testing  
- Integration Testing  
- Documentation Tests  
- Debugging (gdb/lldb)  
- Logging (tracing/log)  
- Profiling & Benchmarks  
  - Criterion  
  - Flamegraphs  
- Linting (Clippy)  
- Formatting (rustfmt)  
- Rust Analyzer (LSP)

## 14. Architecture, CI/CD & DevOps

- Organizing modules & workspace architecture  
- Release profiles  
- Cross-compilation  
- CI/CD pipelines for Rust  
- Dockerizing Rust applications

## 15. Interoperability & Ecosystem Skills

- Python ↔ Rust (PyO3, pyo3-maturin)  
- Node ↔ Rust (NAPI-RS)  
- WASM (WebAssembly)  
- Embedding Rust in other languages  
- Calling Python/C++ from Rust

## Additional Important Topics

- Smart Pointers (Box, Rc, Arc, Weak)  
- Atomics & Memory Ordering  
- Data-Oriented Design (SoA vs AoS, SIMD)  
- cargo features & workspace patterns  
- Publishing crates  
- Popular libraries & frameworks:
  - Tokio
  - Axum
  - Serde
  - Clap
  - SQLX / SeaORM
  - Tonic (gRPC)


## Frameworks

| Framework    | Purpose         |
| ----------- | ---------------- | 
|[Tuari](https://v2.tauri.app/)| to create small, fast, secure, cross-platform applications|
|[dioxus](https://dioxuslabs.com/)| to building fullstack web, desktop, and mobile apps. |
|[Burn](https://burn.dev/)| Deep learning framework like Pytorch for python
|[axum](https://docs.rs/axum/latest/axum/)| web application framework like fastapi in python
|[Yew](https://yew.rs/)| web assembly framework
|[leptos](https://leptos.dev/)| web app framework similar like Yew
|[crux](https://github.com/redbadger/crux)| Cross-platform app development in Rust



## Rust Books

| Online Books| Short Note       |
| ----------- | ---------------- | 
|[Awesome Rust Books](https://github.com/sger/RustBooks)|Offcourse, Awesome ! |
|[Rust Design Patterns](https://rust-unofficial.github.io/patterhgs/)|Important for production system and software principles|
|[Comprehensive Rust](https://google.github.io/comprehensive-rust/)|free Rust course developed by the Android team at Google
|[Easy Rust](https://dhghomon.github.io/easy_rust/Chapter_0.html)|Explained little more simple language and easy to follow. 
|[Fast Track to Rust](https://freddiehaddad.github.io/fast-track-to-rust/#welcome-to-fast-track-to-rust)|Learn while building grep-like program, called rustle
|[Rust 101](https://www.ralfj.de/projects/rust-101/main.html)|Outdated may be but i liked the explanations.


## Design Patterns

| Category    | Pattern          | Rust Example                                      |
| ----------- | ---------------- | ------------------------------------------------- |
| Creational  | Singleton        | [Rust Code](design-patterns/singleton_pattern.rs) |
|             | Builder          | [Rust Code](design-patterns/builder_pattern.rs)   |
|             | Abstract Factory | [Rust Code](design-patterns/factory_pattern.rs)   |
| Behavioural | Command          | [Rust Code](design-patterns/command_pattern.rs)   |
|             | Observer         | [Rust Code](design-patterns/observer_pattern.rs)  |
|             | Strategy         | [Rust Code](design-patterns/strategy_pattern.rs)  |
|             | State            | [Rust Code](design-patterns/state_pattern.rs)     |
|             | Iterator         | TODO                                              |
| Structural  | Adapter          | [Rust Code](design-patterns/adaptor_pattern.rs)   |
|             | Decorator        | [Rust Code](design-patterns/decorator_pattern.rs) |
|             | Facade           | TODO                                              |
|             | Proxy            | TODO                                              |

### Commands Snippets

```rust
cargo new rust-learning --bin


cargo run
cargo new utils --lib
cargo run --example <filename>

cargo build --bin another-bin

cargo run --bin rust-learning

cargo run --bin another-bin
```


### Top Macros

`TODO: will have to convert into Macro.md file`

- #[allow(dead_code)]


---


💛 Support My Open source Work

<a href="https://buymeacoffee.com/untitledhuman" target="_blank">
    <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 50px;">
</a>



