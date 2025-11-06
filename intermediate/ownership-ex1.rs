fn main() {
    let address_old = String::from("123 Main St");
    println!("The address is: {}", address_old);
    let address_new = address_old; // ownership moved
    println!("The new address is: {}", address_new);
    println!("The address is: {}", address_old); // this will cause an error
    // The value remains, but it's the accessibility that's changed, not the value.

}

// Error: borrow of moved value: `address_old`
//      println!("The address is: {}", address_old); // this will cause an error
//|                                    ^^^^^^^^^^^ value borrowed here after move
