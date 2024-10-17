// Bring the library crate into scope
use utils::greet;
use utils::math;

fn main() {
    let sum = math::add(5, 3);
    let product = math::multiply(4, 6);

    println!("The sum is: {}", sum);
    println!("The product is: {}", product);

    println!("{}", greet::greet_user("Rustacean"));
}
