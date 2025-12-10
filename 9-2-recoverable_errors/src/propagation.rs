use std::fs::File;
use std::io::{self, Read};

pub fn read_from_file() -> Result<String, io::Error> {
    let mut file = match File::open("username.txt") {
        Ok(handle) => handle,
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _question_operator() -> Result<String, io::Error> {
    let mut username = String::new();
    // Return error value if encountered while using from trait to convert to the specified error type
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Same as above function just using a concise syntax
fn _read_to_string() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}
