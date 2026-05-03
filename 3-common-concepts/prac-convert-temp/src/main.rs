use std::io::{Write, stdin, stdout};

fn main() {
    loop {
        println!(
            "Convert temperature\n\t1. Celsius to Fahrenheit\n\t2. Fahrenheit to Celsius\n\t3. Exit"
        );
        let mut input = String::new();
        print!("Enter option: ");
        stdout().flush().expect("Failed to flush stdout");
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(num) => {
                        if num == 1 {
                            let temperature = get_temperature("Enter temperature in Celsius: ");
                            c_to_f(temperature);
                        } else if num == 2 {
                            let temperature = get_temperature("Enter temperature in Fahrenheit: ");
                            f_to_c(temperature);
                        } else if num == 3 {
                            println!("Closing");
                            break;
                        } else {
                            println!("Please enter a valid option (1 or 2)");
                        }
                    }
                    Err(err) => {
                        println!("Enter a valid number, {:?}", err.to_string());
                        continue;
                    }
                };
            }
            Err(err) => {
                println!("Error reading input: {:?}", err.to_string());
                continue;
            }
        }
    }
}

fn get_temperature(str: &str) -> f64 {
    loop {
        print!("{str}");
        stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(err) => {
                println!("Enter a valid number, {:?}", err.to_string());
                continue;
            }
        }
    }
}

fn c_to_f(c: f64) {
    println!("Celsius: {c}, Fahrenheit: {}", (c * 9.0 / 5.0) + 32.0);
}
fn f_to_c(f: f64) {
    println!("Celsius: {f}, Fahrenheit: {}", (f - 32.0) * 5.0 / 9.0);
}
