fn main() {
    let mut read_count = 10;

    while read_count > 0 {
        println!("Reading... {} reads left", read_count);
        read_count -= 1;
    }
}