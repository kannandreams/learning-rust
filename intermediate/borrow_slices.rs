// Slices let you reference collection parts, not the entire collection.

fn main() {
    let s = String::from("hello");
    let slice = &s[0..2]; // borrow a slice of the string
    println!("Slice: {}", slice);
}