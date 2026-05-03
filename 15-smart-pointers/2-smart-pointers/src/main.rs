use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    // let y = &x;
    
    // Box copies x value instead of directly referencing
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // Have to implement deref trait
    assert_eq!(5, *y);
    
    let m = MyBox::new(String::from("Name1"));
    // Deref coercion dereferences &MyBox value to &String then &String gets derefed to return &str (string slice)
    hello(&m);
    // Without deref coercion
    hello(&(*m)[..]);
    // If deref trait wasnt implemented
    hello(&(*m.0)[..]);
}

fn hello(name: &str) {
    println!("Hello {name}");
}
