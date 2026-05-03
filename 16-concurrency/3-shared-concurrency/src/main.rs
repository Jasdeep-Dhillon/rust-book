use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // Create a mutex lock
    let m = Mutex::new(5);
    {
        // Get mutex lock and update the value within
        // *m.lock().unwrap() = 6;
        match m.lock() {
            Ok(mut val) => {
                *val = 6;
            }
            Err(e) => {
                println!("Error unlocking mutex lock {e}")
            }
        }
    }
    println!("m: {m:?}");

    // Arc (Atomic Reference Count) implements the Send and Sync alongside atomic operations behaves similar to Rc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());

    deadlock();
}

fn deadlock() {
    let val = Mutex::new(10);
    // If only acquiring lock and unwrapping later then the lock gets dropped
    let n1 = val.lock();
    // Ends up dropping the lock because result value gets dropped after since
    // it is not being referenced
    println!("Got lock n1 {}", n1.unwrap());
    let n2 = val.lock();
    println!("Got lock n2 {}", n2.unwrap());

    println!("---------Deadlock---------");
    let n1 = val.lock().unwrap();
    println!("Got lock for n1 {n1}");
    let n2 = val.lock().unwrap();
    // println!("Got lock for n1 {}", n1.unwrap());
    println!("Got lock for n2 {n2}");
}

// Couldn't get thread to deadlock
// fn deadlock_threads() {
// let mutex1 = Mutex::new(1);
// let mutex2 = Mutex::new(2);

// let handle1 = thread::spawn(move || {
//     let _ = mutex1.lock().unwrap();
//     println!("Acquired mutex1 lock in thread 0");
//     thread::sleep(Duration::from_millis(5));
//     let _ = mutex2.lock().unwrap();
// });
// let handle2 = thread::spawn(move || {
//     let _ = mutex2.lock().unwrap();
//     println!("Acquired mutex2 lock in thread 1");
//     thread::sleep(Duration::from_millis(5));
//     let _ = mutex1.lock().unwrap();
// });
// }
