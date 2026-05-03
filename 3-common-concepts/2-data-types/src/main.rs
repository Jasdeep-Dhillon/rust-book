fn main() {
    println!("Hello, world!");

    let _: u32 = 42;
    let _: i32 = -42;
    // Hexadecimal
    let _: u32 = 0x32;
    // Octal
    let _: u32 = 0o27;
    //Binary
    let _: u32 = 0b101010;
    //Byte (u8 only)
    let _: u8 = b'a';

    // Floating
    let _ = 2.718281828459045;

    // Operators
    // Addition +, Subtraction -,Multiplication *, Division /, Remainder / Modulus %,

    // Booleans
    let is_true: bool = true;
    if is_true {
        println!("It's true!");
    }

    // Characters
    let _: char = 'z';
    let _: char = '💀';

    // Strings
    let _: &str = "Hello, Rust!";
    let _: String = String::new();
    let _: &'static str = "z";

    // Tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    // Destructing with types
    let (x, y, z): (i32, f64, u8) = (x, y, z);
    println!("The value of y is: {0},{1},{2}", x, y, z);

    // Arrays
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("The first value of arr is {}", arr[0]);
}
