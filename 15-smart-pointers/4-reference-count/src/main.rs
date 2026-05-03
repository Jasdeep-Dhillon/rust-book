use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
impl Drop for List {
    fn drop(&mut self) {
        println!("Destroying list {:?}", &self);
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("b: {b:?}, c: {c:?}");
    }
    println!("count after c out of scope = {}", Rc::strong_count(&a));   
}
