use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b: List =Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;
    
    if let Cons(b_value, _) = &b {
        b_value.replace(67);
        println!("b_value: {}", b_value.borrow());
    }
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
    
    
    let mut list = &b;
    loop {
        let Cons(b_value, remaining) = list else {
            break;
        };
        
        list = &(*remaining);
        println!("b list iterate: {}", b_value.borrow());
    }
}
