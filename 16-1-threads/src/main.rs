use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..1000 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..500 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
    
    match handle.join() {
        Ok(_) => println!("Completed all tasks by thread"),
        Err(_) => println!("Failed to finish executing thread")
    };
    
    // Using values in threads
    let v = vec![1,2,3];
    
    // Inferred return with Vec<i32> value
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
        v
    });
    // Cannot use v as closure currently owns it
    // println!("v: {v:?}");
    
    // Can get value back after the thread is done executing
    let v = handle.join().unwrap();
    println!("Taking ownership back of v after completing thread {v:?}");
}
