use std::fs::File;
use std::io::{ErrorKind, Read};

mod propagation;
use propagation::read_from_file;

fn main() {
    let mut file_handle = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new_file) => new_file,
                Err(e) => panic!("Error creating file {e:?}"),
            },
            _ => {
                panic!("Error opening the file {error:?}")
            }
        },
    };

    let mut content = String::new();

    match file_handle.read_to_string(&mut content) {
        Ok(_) => {
            let content = content.split_whitespace();
            for line in content {
                println!("{line}")
            }
        }
        Err(error) => panic!("Error reading from file {error:?}"),
    }
    // println!("{file_handle:?}");
    match read_from_file() {
        Ok(username) => println!("{username}"),
        Err(e) => panic!("{e:?}"),
    }
}
