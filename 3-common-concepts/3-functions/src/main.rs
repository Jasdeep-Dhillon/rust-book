fn main() {
    println!("Hello, World");
    another_function();
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // returns the value of x + 1
    };
    println!("The value of y is: {}", y);
    println!("The value of five() is: {}", five());
    println!("The value of plus_one(5) is: {}", plus_one(5));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn another_function() {
    println!("Printing from another funciton");
}

fn five() -> i32 /* return type */ {
    5 // no semicolon returns the expression
}

// return expression with parameter
fn plus_one(x: i32) -> i32 {
    x + 1
}
