fn main() {
    // Mutable borrow example
    let s = String::from("hello");
    let borrowed_s = &s; // borrow s
    println!("Original String: {}", s);
    println!("Borrowed String: {}", borrowed_s);

    // Immutable borrow example
    let mut s_mut = String::from("hello");
    let borrowed_s_mut = &mut s_mut; // mutable borrow
    borrowed_s_mut.push_str(", world");
    println!("Mutably Borrowed String: {}", borrowed_s_mut);

    // Immutable borrow example - only one mutable reference allowed
    let s2 = String::from("hello");
    let borrowed_s2 = &mut s2; // first immutable borrow
    // let borrowed_s2_again = &mut s2; // this would cause an error
    println!("Immutable Borrowed String: {}", borrowed_s2);

    // Immutable borrow example - multiple immutable references allowed but not alongside a mutable borrow
    let s3 = String::from("hello");
    let borrowed_s3_1 = &s3; // first immutable borrow
    let borrowed_s3_2 = &s3; // second immutable borrow
    println!("First Immutable Borrowed String: {}", borrowed_s3_1);
    println!("Second Immutable Borrowed String: {}", borrowed_s3_2);
    let borrowed_s3_3 = &mut s3; // third mutable borrow - this would cause an error
    println!("Third Immutable Borrowed String: {}", borrowed_s3_3);

}