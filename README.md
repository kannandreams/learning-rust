
# Learning Guide for Rustaceans 🦀

## 1. Getting Started

* Learning Guide for Rustaceans
* Rust tool chain (Rust’s build system and package manager)

## 2. Basics / Core Concepts

* Primitive Data Types
* Complex Data Types (Arrays, Tuples, Slices, Strings)
* Variables & Mutability
* Constants & Static Variables
* Functions
* Control Flow (if, loop, while, for)

## 3. Ownership, Borrowing & Memory

* Ownership & Borrowing
* References & Mutable References
* Clone
* Copy, Lifetimes
* Memory Safety & Rules

## 4. Data Structures

* Structs
* Enums
* Tuples
* Collections (Vec, HashMap, HashSet, etc.)
* Implementation of Custom Types
* Iterators
* Closures

## 5.  Associated Types, Modules & Packages

* Modules & Paths
* Crates
* Packages / Workspaces
* Standard Library Overview

## 6. Advanced Type System

* Generics
* Traits
* Trait Objects
* Type Aliases
* Pattern Matching & Destructuring
* Concurrency, Arc, Send
* Slices, Arrays, External Dependencies
* Mutex, Interior Mutability (cont.), RwLock, Sync
* Unsafe Rust, Drop

## 7. Error Handling

* Exception Handling (`Result` & `Option`)
* Panic and Unwinding
* Custom Errors

## 8. Programming Paradigms

* Object-Oriented Programming Concepts
* Functional Programming Concepts
* Closures & Higher-Order Functions

## 9. Concurrency & Asynchronous Programming

* Threads & Concurrency
* `async` / `await`
* Channels & Message Passing
* Mutex & RwLock

## 10. Macros & Metaprogramming

* Declarative Macros (`macro_rules!`)
* Procedural Macros
* Attributes

## 11. Testing & Debugging

* Unit Testing
* Integration Testing
* Documentation Tests
* Debugging & Logging

## 12. Patterns & Idioms

* Pattern Matching (`match`)
* `if let` / `while let`
* Iterator Patterns
* Common Rust Idioms & Best Practices

## Suggested Missing Topics

* File I/O and Serialization (JSON, CSV, etc.)
* Networking Basics
* Foreign Function Interface (FFI)
* Unsafe Rust (when and how to use it safely)
* Performance Profiling & Benchmarks
* Rust Tooling (Clippy, Rustfmt, Rust Analyzer)

## Miscs

* Profiling
* Debugging
* Unit Testing
* CI/CD
* Integrate Python ( Invoke rust in Python )

## Frameworks

* [Tuari](https://v2.tauri.app/)
    - Create small, fast, secure, cross-platform applications
* [dioxus](https://dioxuslabs.com/)
    - Dioxus is the Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time.
* [Burn](https://burn.dev/)
    - Deep learning

## Rust Books

- [Awesome Rust Books](https://github.com/sger/RustBooks) - Offcourse, Awesome ! 

- [Rust Design Patterns](https://rust-unofficial.github.io/patterhgs/) - Important for production system and software principles

- [Comprehensive Rust](https://google.github.io/comprehensive-rust/) - free Rust course developed by the Android team at Google

- [Easy Rust](https://dhghomon.github.io/easy_rust/Chapter_0.html) - Explained little more simple language and easy to follow. 

- [Fast Track to Rust](https://freddiehaddad.github.io/fast-track-to-rust/#welcome-to-fast-track-to-rust) - Learn while building grep-like program, called rustle

- [Rust 101](https://www.ralfj.de/projects/rust-101/main.html) - Outdated may be but i liked the explanations.


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

### Learning Courses

- Code Crafters ( build real time projects )

