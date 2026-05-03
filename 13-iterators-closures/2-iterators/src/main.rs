fn main() {
    let v1 = vec![1, 2, 3];
    // iter() Returns immutable references to values
    // into_iter() takes ownership of self
    // iter_mut() returns mutable references 
    let v1_iter = v1.iter();
    
    for v in v1_iter {
        println!("Got: {v}");
    }
}

