fn main() {
    for i in 1..=5 {
        println!("Iteration number: {}", i);
    }

    for item in ["apple", "banana", "cherry"].iter() {
        println!("Fruit: {}", item);
    }

    for i in (1..=10).step_by(2) {
        println!("Odd number: {}", i);
    }

    for i in (1..=5).rev() {
        println!("Countdown: {}", i);
    }

    for (index, value) in ["red", "green", "blue"].iter().enumerate() {
        println!("Color {}: {}", index + 1, value);
    }

    for c in "rust".chars() {
        println!("Character: {}", c);
    }

}