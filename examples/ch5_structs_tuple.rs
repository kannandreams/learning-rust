// This is used to print the struct and Debug is a trait
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
        // We can also create struct with name fields - called tuple structs
        let rect = (30, 50);

        println!("The area of the rectangle is: {}", area(rect));

        // We can also create struct with named fields - called struct
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("The area of the rectangle is: {}", area_struct(&rect1));
        println!("rectangle is: {:#?}", rect1); // {:?} and  #[derive(Debug)] is used to print the struct
    
}

fn area(dimensions : (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}