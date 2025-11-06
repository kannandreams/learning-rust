fn main() {
    let blog: &str = "When engineers meet AI"; 
    let status: bool = true; // boolean
    let blog_focus = ("AI", "Engineering"); // tuple - type inferred from the values
    let blog_sections  = [ "AI", "Engineering", "Rust" ]; // array - type inferred from the values
    println!("{}", blog);
    println!("{}", status);
    println!("{:?}", blog_focus);
    println!("{:?}", blog_sections);
}

