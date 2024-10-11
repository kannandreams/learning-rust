use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess!");

    

    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..10);
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
    
        }
    }
    

}



