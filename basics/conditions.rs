fn main() {
    let salary = 1000;
    // rust expects expressions to return a same type ( str ) in all branches
    if salary < 500 {
        println!("Low income");
    } else if salary >= 500 && salary < 2000 {
        println!("Middle income");
    } else {
        println!("High income");
    }

    let loan = if salary < 1500 { 500 } else { 2000 };
    println!("Loan amount: {}", loan);
}