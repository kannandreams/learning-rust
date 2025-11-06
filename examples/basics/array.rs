fn main(){
    let person:[&str; 3 ] = [ "der", "die", "das" ]; // array - type inferred from the values
    println!("{:?}", person);
    println!("{}", person[1]);
}