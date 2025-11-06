#### Ownership in Rust: 
- Rust has a unique ownership model for memory safety. 
- Each data piece has one owner, and using a variable after transferring its ownership leads to an error, ensuring that data isn't modified from multiple locations at the same time.
- Complex data type ownership: Types like String strictly adhere to Rust's ownership model. Once their ownership is transferred to another variable, the original becomes unusable.


- Rust categorize the data tyes into `Copy` and `non-Copy`
- Integer types implement the Copy trait, meaning their values are copied
- `Copy`: Simple scalar types like integers, bool, and characters.
- `Non-Copy`: Types such as String or Vec.



#### References - Borrowing
Rust provides a borrowing system to access data without transferring ownership
- The borrow symbol is `&`
- Borrowing has two types: `immutable` and `mutable`
- An immutable borrow lets you read data, but not change it.
- symbol is `&mut`
- A mutable reference lets you alter data.
- Only one mutable reference is allowed within a scope. This ensures no data races.
- Multiple immutable borrows can coexist, but not alongside a mutable borrow.

#### Dangling references
- Dangling is when a reference's data gets freed while the reference still exists.
- Rust ensures references never "dangle".

In plain English:

You have a reference (like a pointer) to some data.
The data is destroyed (goes out of scope, or freed from memory).
But your reference still points to that (now invalid) memory.
That reference is called a dangling reference or dangling pointer.



