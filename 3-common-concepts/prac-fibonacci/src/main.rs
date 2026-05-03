use std::io::{Write, stdin, stdout};

fn main() {
    print!("Please enter the fibonacci iterations: ");
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    let nth: u32 = input.trim().parse().expect("Please enter a valid number");

    // if iterations >= 3000 {
    //     println!("Please enter a number less than or equal to 3000");
    //     return;
    // }

    // let mut a: [i32; 3000] = [-1; 3000];
    // for index in 0..iterations {
    //     a[index] = index.try_into().unwrap();
    //     println!("{index}: {}", a[index]);
    // }
    let mut first = 1;
    let mut second = 1;
    for _index in 2..nth {
        let next = first + second;
        first = second;
        second = next;
        println!("{}: {second}", _index + 1);
    }
    println!("Fibonacci at position {nth} is {second}");

    const ROOT_5: f64 = 2.23606797749979;
    let result =
        ((1.0 + ROOT_5).powi(nth as i32) / ((2_i32).pow(nth) as f64 * ROOT_5)).round() as u32;
    println!("Fibonacci at position {nth} using formula is {result}");
}
