fn main() {
    let mut attempts = 0;

    loop {
        println!("Connecting...");
        attempts += 1;
        if attempts >= 3 {
            println!("Failed after 3 attempts.");
            break;
        }
    }
}
