fn main() {
    let s1 = String::from("hello");
    // Passes a reference instead, ownership does not get transferred
    // Reference is a pointer to actual data
    let len = calculate_length(&s1);
    println!("THe length of {s1} is {len}");

    // Passing a mutable reference
    change_string(&mut String::from(s1));
    
    let mut s = String::from("Hello");
    let r1 = &mut s;
    // Can only have one mutable reference present at a time
    // let r2 = &mut s;
    
    println!("{r1} r2 is invalid");
    
    let r1 = &s;
    let r2 = &s;
    // Can have multiple non-mutable references
    // Mutable reference can only exist when no other references exist
    // let r3 = &mut s;
    println!("{r1} {r2}");
    // Works because r1 and r2 are going to be accessed after this line
    let r3 = &mut s;
    println!("{r3}");
    
    let _s = no_dangle();
    
}

fn calculate_length(s1: &str) -> usize {
    s1.len()
}

fn change_string(s: &mut String) {
    s.push_str(" World");
    println!("New Value \"{s}\"");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// Cannot return reference when you are not borrowing any references
// fn no_dangle_error() -> &String {
//     let s = String::from("hello");
//     &s
// }