mod employee;
mod median_mode;
mod pig_latin;
// use std::collections::HashMap;
use std::collections::BTreeMap;

use employee::run;
use median_mode::get_result;
use pig_latin::transform;

fn main() {
    prac1();
    prac2();
    prac3();
}

fn prac1() {
    let mut list = vec![
        88, 12, 12, 54, 30, 99, 12, 77, 88, 10, 1, 44, 55, 66, 77, 88, 99, 100, 11, 22, 33, 44, 55,
        66, 77, 88, 99, 100, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
        44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
        67, 68, 69, 70, 71, 72, 73,
    ];
    let result = get_result(&mut list);
    println!(
        "List has median {} and mode {}",
        result.get_median().unwrap_or_default(),
        result.get_mode().unwrap_or_default()
    );
}

fn prac2() {
    let mut string = String::from("first");
    println!("{}", transform(&mut string));
    let mut string = String::from("apple");
    transform(&mut string);
    println!("{string}",);
}

fn prac3() {
    let mut organization: BTreeMap<String, Vec<String>> = BTreeMap::new();
    organization.insert(
        String::from("engineering"),
        vec![String::from("e1"), String::from("e2"), String::from("e3")],
    );
    organization.insert(
        String::from("sales"),
        vec![String::from("e1"), String::from("e2"), String::from("e3")],
    );
    run(&mut organization);
}
