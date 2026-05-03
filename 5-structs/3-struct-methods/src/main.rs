#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Methods related to Rectangle struct
impl Rectangle {
    // Pass reference to self
    // Can use &self shorthand instead of self: &Self or rectangle: &Rectangle
    // Self is an alias for the type of impl block
    // Need self as the first parameter
    // Use &mut self if modifying the variable
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn something() -> String {
        String::from("Returning")
    }
}

impl Rectangle {
    fn multiple_impl() {
        println!("Function inside another implement block");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has nonzero width; {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{}", Rectangle::something());
    let square = Rectangle::square(3);
    dbg!(&square);
    Rectangle::multiple_impl();
}
