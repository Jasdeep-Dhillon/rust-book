fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    println!(
        "{}",
        match third {
            Some(third) => format!("Third element is {}", third),
            None => String::from("There is no third element"),
        }
    );

    // Will cause panic
    // let _does_not_exist = &v[100];
    // Handles the error and returns an option instead
    let _does_not_exist = v.get(100);
    let third = &v[2];
    // Only one reference exists to v currently so third is available
    // Can have as many read only references
    let _one = &v[0];
    let _second = &v[1];
    println!("The third element is {third}");
    // Mutating data will invalidate other references to data as the vector might reallocate to increase its size
    // Cannot use third anymore because vector has been changed
    v.push(7);
    let removed = v.remove(v.len() - 2);
    println!("Removed: {removed}");
    // println!("The third element is {third}");

    iterate_vector();
    multiple_types();
}

fn iterate_vector() {
    let mut v = vec![100, 32, 67];
    for i in &mut v {
        *i += 67;
    }
    for i in &v {
        println!("{i}");
    }
}

// #[derive(Debug)]
enum VecTypes {
    Int(i32),
    Float(f64),
    Text(String),
}

fn multiple_types() {
    let row = vec![
        VecTypes::Int(3),
        VecTypes::Float(3.14),
        VecTypes::Text(String::from("value")),
    ];

    for i in &row {
        println!(
            "{}",
            match i {
                VecTypes::Int(n) => n.to_string(),
                VecTypes::Float(f) => f.to_string(),
                VecTypes::Text(t) => t.clone(),
            }
        );
    }
}
