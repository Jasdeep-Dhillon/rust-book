fn main() {
    let s = String::from("hello");
    let s1 = s; // Moves s ownership to s1
    
    let a = 5;
    let _b = a; // Created copy instead
    
    // Creates a copy of s1
    let _clone = s1.clone();
    takes_ownership(_clone);
    // Will throw error as ownership has been transferred
    // println!("The value of string is {s}");
    let s = takes_and_returns_ownership(s1);
    println!("The value of string is {s}");
}

fn takes_ownership(passed_s: String) {
    println!("Passed s is {passed_s}");
}

fn takes_and_returns_ownership(s: String) -> String{
    println!("Passed s is {s}");
    s
}
