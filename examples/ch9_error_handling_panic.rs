fn array_allocation() {
    let v = vec![1, 2, 3];
    v[99];
}

fn main() {
    array_allocation();
    //panic!("crash and burn");
}