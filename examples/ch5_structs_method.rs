// This is used to print the struct and Debug is a trait
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // first argument is always self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

    // Associated function not taking self as argument - similar to static method in other languages
    // rust allow multiple impl blocks
    impl Rectangle {

        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }


fn main() {
       
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 20,
            height: 30,
        };

        let rect3 = Rectangle::square(25);

        println!("The area of the rectangle is: {}", rect1.area());
        println!("rectangle is: {:#?}", rect1); // {:?} and  #[derive(Debug)] is used to print the struct

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

        println!("square is: {:#?}", rect3);
    
}
