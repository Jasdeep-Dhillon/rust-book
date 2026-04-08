use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        match tx.send(val) {
            Ok(_) => println!("Sent data successfully"),
            Err(e) => println!("Could not send data to receiver {e}"),
        };
        // Ownership gets transferred to receiver
        // println!("val is {val}");
    })
    // Joining and unwrapping would help run it before the receiving block
    // .join().unwrap()
    ;

    // recv() blocks the thread until something is received
    // try_recv() doesn't block the thread and can be used for polling
    // Ends up executing before the thread could spawn resulting in error
    loop {
        let received = match rx.try_recv() {
            Ok(val) => {
                println!("Got: {val}");
                val
            }
            Err(e) => {
                println!("Error receiving: {e}");
                // tx.send(String::from("Something")).unwrap();
                break;
            }
        };
        println!("Received: {received}");
    }

    println!("--------------------------------------------");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        tx
    })/* .join().unwrap() */;

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Can nest thread multiple times
    thread::spawn(move || {
        thread::spawn(move || {
            thread::spawn(move || {
                thread::spawn(move || {
                    tx2.send(String::from("nested threads")).unwrap();
                });
            });
        });
    });
    // Doesn't run if not joined, doesn't receive the last block otherwise
    // thread::spawn(move || {
    for received in rx {
        println!("Got: {received}");
    }
    // loop {
    //     match rx.recv_timeout(Duration::from_secs(5)) {
    //         Ok(msg) => println!("Got: {msg}"),
    //         Err(e) => {
    //             println!("Error: {e}");
    //             break;
    //         }
    //     }
    // }
    // });

    // Never receives data from this
    // Main thread is active and waiting to receive
    thread::spawn(move || {
        tx3.send(String::from("after receive block")).unwrap();
    })
    .join()
    .unwrap();
}
