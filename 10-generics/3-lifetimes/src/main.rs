use std::fmt::Display;

struct ImportantExcerpt<'a> {
    _part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Uses the smaller scope of s2 and longest for lifetime which are valid
    let s1 = String::from("abc");
    {
        let s2 = String::from("qwerty");
        let longest = longest(&s1, &s2);
        println!("{longest} is the longer string");
    }

    // Does not work because s2 ends scope ends before s1 and longer
    // let s1 = String::from("abc");
    // let longer: &str;
    // {
    //     let s2 = String::from("qwerty");
    //     longer = longest(&s1.as_str(), &s2.as_str());
    // }
    // println!("{longer} is the longer string");
    //
    // This does not work either because the return slice lifetime is longer than the arguments lifetimes
    // let longer: &str;
    // {
    //     let s1 = String::from("abc");
    //     let s2 = String::from("qwerty");
    //     longer = longest(&s1.as_str(), &s2.as_str());
    // }
    // println!("{longer} is the longer string");
    let _i;
    {
        let novel = String::from("call me ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not get next value");
        _i = ImportantExcerpt {
            _part: first_sentence,
        };
    }
    // println!("{}", i.part);

    // Have lifetime throughout the entire program
    // String literals by default have static lifetime
    let _: &'static str = "Static Lifetime";
    let _ = "some";
}

// &'a i32
// &'a mut i32
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn _longest2<'a>(x: &'a str, y: &str) -> &'a str {
    if y.len() > 3 {
        println!("Y is longer than 3");
    }
    x
}

// Using generic, trait bounds and lifetimes
fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoucement {ann}");
    if x.len() > y.len() { x } else { y }
}
