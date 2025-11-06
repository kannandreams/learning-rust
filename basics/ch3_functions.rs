fn main() {
    my_function("Sum");
    let sum = my_sum(1, 2);
    println!("The sum is {}", sum);

}

// snake_case
fn my_function(name: &str) {
    println!("This is my function! {}", name);
}

fn my_sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
    return sum;
}