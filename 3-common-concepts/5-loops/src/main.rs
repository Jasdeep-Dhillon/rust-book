fn main() {
    let mut number = 0;
    let number = loop {
        println!("again!");
        number += 1;
        if number > 5 {
            // Returning value from loop
            break number;
        }
    };

    println!("The value of number is {number}");

    let mut count = 0;
    // Nested and labeled loops
    let result = 'counting: loop {
        println!("Count: {count}");

        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");
            // Break condition for inner loop
            if remaining == 1 {
                break;
            }
            // Break condition for outer loop using label
            if count == 2 {
                break 'counting count * 2;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("End count: {count}, result: {result}");

    // While Loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Finished {number}");

    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // For each loop
    for element in a {
        println!("the value is: {element}");
    }

    for num in (1..5).rev() {
        print!("{num} ");
    }
    println!("LIFTOFF!!!");
}
