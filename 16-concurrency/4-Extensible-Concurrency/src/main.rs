use std::{fmt::Error, sync::Arc};

struct Test {
    val: Arc<u32>,
}

fn main() {
    let test = Arc::new(Test { val: Arc::new(10) });

    let test1 = Arc::clone(&test);
    let handle1 = std::thread::spawn(move || {
        println!("Test value: {}", test1.val);
        Error::default()
    });
    let test2 = Arc::clone(&test);
    let handle2 = std::thread::spawn(move || {
        println!("Test value: {}", test2.val);
    });

    // Unhandled error
    let _handle1 = handle1.join();
    let _handle2 = handle2.join();
}
