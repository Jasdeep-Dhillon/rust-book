fn main() {
    // Immutable variable
    // let x = 5;
    // Mutatable variable
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    // Constant variable
    const CONSTANT: u32 = 1_00_00_0;
    println!("the value of constant is {CONSTANT}");

    // Shadowing
    let x = 6;
    let x = x + 100;
    {
        let x = x * 2;
        println!("the value of x in inner scope is {x}");
    }
    println!("the value of x is {x}");
}
