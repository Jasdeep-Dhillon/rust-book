use closures::Rectangle;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Unwrap or else implements FnOnce trait
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pf1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pf1);
    println!("The user with preferences {user_pf1:?} gets {giveaway1:?}");

    let user_pf2 = None;
    let giveaway2 = store.giveaway(user_pf2);
    println!("The user with preference {user_pf2:?} get {giveaway2:?}");

    // Closure with type annotations
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    };

    let result = expensive_closure(2);
    println!("Result of expensive closure is {result}");

    // Both produce the same result
    // Closure is less verbose and can type infer
    // because functions can be exposed and need to have a set return type for the caller
    fn _add_one(x: u32) -> u32 {
        x + 1
    }
    let add_one_closure = |x| x + 1;

    let result = add_one_closure(1);
    println!("Add one closure result: {result}");
    // Does not work because the inferred type are already set as i32 for argument and return type
    // let result = add_one_closure(String::from("abc"))

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let borrow_closure = || println!("From closure: {list:?}");
    borrow_closure();
    let list = 1;
    borrow_closure();
    println!("{list}");

    let mut list = vec![1, 2, 3];
    let mut mutable_borrow = || list.push(4);
    mutable_borrow();
    println!("{list:?}");
    // Does not work because list would violate borrow checker with one mutable and one immutable reference
    // mutable_borrow();
    let list = 1;
    println!("{list}");

    // move keyword moves the ownership of variable to closure
    let list = vec![1, 2, 3];
    // let closure = move || println!("{list:?}");
    // closure();
    // closure();
    std::thread::spawn(move || println!("{list:?}"))
        .join()
        .expect("Failed to spawn thread");
    // Cannot access list anymore since ownership was moved to closure and the closure is out of scope
    // println!("{list:?}");

    // FnOnce: Closure that returns the ownership
    // Can only be called once
    test(|| print!("FnOnce Closure"));
    // FnMut: Closure that mutates the passed argument
    // Can be called multiple times
    //
    // Fn: Closure that don't mutate, return or take ownership of the passed argument
    // Can be called multiple times concurrently

    let rectangles = Rectangle::sorted_rectangles_by_width();
    println!("{rectangles:?}");
}

fn test(argument: impl FnOnce()) {
    argument();
}
