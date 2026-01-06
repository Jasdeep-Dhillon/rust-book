fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true")
    } else {
        println!("Condition was false");
    }

    if number != 3 {
        println!("Number was not zero");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is something else");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");
}
