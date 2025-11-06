fn main() {
    let person: (&str, &str, &bool, i32) = ("foo", "bar", &true, 30); // tuple - type inferred from the values
    person.3; // accessing tuple element by index
    let (age,name) = (person.3, person.0); // destructuring tuple into variables
    println!("{:?}", person);
    println!("{}", person.3);
    println!("Name: {}, Age: {}", name, age);
}

