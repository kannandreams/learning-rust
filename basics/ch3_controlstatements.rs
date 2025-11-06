fn main(){
    let number = 2;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;

    // general loop
    loop {
        counter += 1;
        println!("The value of counter is: {}", counter);
        if counter == 10 {
            break;
        }
    }

    // while loop
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }

}