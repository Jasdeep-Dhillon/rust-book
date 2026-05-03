#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // Debug Info
    // println!("{rect1:?}");
    
    // Debug Macro
    // Takes ownership of the variable and returns it
    // Prints the file name, line number
    // Prints to stderr instead of stdout
    let rect1 = dbg!(rect1);
    println!("The area of rectangle is {} square pixels", area(&rect1));
    
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    
    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
