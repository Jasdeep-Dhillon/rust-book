#[derive(Debug, Clone)]
enum LinkedList {
    Cons(i32, Box<LinkedList>),
    Nil,
}

use crate::LinkedList::{Cons, Nil};

fn main() {
    // Allocate memory on the heap
    let b = Box::new(5);
    println!("b: {b}");

    let mut list = Box::new(
        Cons(1, Box::new(
            Cons(2, Box::new(
                Cons(3, Box::new(
                    Nil)
                ))
            ))
        )
    );

    println!("{list:?}");
    
    loop {
        let Cons(value, remaining) = *list else {
            break;
        };
        list = remaining;
        println!("value: {value}");
    }
}
