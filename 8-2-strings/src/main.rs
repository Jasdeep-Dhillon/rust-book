fn main() {
    let _s = "".to_string();
    println!("The value of string is '{_s}'");
    
    let _s = String::new();
    println!("The value of string is '{_s}'");
    
    let _s = String::from("Hello");
    let _s = "Hello".to_string();
    
    print_bytes(&String::from("السلام"));
    print_chars(&String::from("नमस्ते"));
    
    let mut s = String::from("Hello");
    s.push_str(" World");
    let exclamation = "!";
    // Add (+) takes ownership of s and returns a string
    // 2nd parameter must pass a reference to the string instead of passing string 
    s = s + &exclamation;
    println!("The value of string is {s}");
    
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from(",");
    
    let s = format!("{s1}{s3} {s2}!");
    println!("The value of string is {s}");
    
    // Do not index into strings directly as they contain UTF-8 encoded values
    // String is just a vector of u8 values
    // Can use range slicing on string to get characters 
    let s = String::from("Здравствуйте");
    let answer =  &s[0..4];
    println!("First index value {answer}");
    
}

fn print_chars(string : &String) {
    for char in string.chars() {
        println!("{char}");
    }
}

fn print_bytes(string: &String) {
    for byte in string.bytes() {
        println!("{byte}");
    }
}