// Using multiple generic types
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// Any placeholder for generic is acceptable
// Doesn't need to be the same as declaration
impl<R, U> Point<R, U> {
    fn get_x(&self) -> &R {
        &self.x
    }

    fn get_type(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }

    fn _get_y(&self) -> &U {
        &self.y
    }
}

// Cannot implement two get_y methods for function overloading
// Defining a function with generic type for x and f32 constraint on y
impl<T> Point<T, f32> {
    fn get_y_f32(&self) -> f32 {
        println!(
            "This {} has f32 value as its y value",
            std::any::type_name::<Self>()
        );
        self.y
    }
}
// Defining a function with constraint of f32 type for x and y values
impl Point<f32, f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T1, U1> Point<T1, U1> {
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T1, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let numbers = vec![132, 1, 554, 645, 67];
    println!("The largest number is {}", largest(&numbers));

    let letters = vec!['a', 'q', 'h', 't'];
    println!("The largest number is {}", largest(&letters));

    let p1 = Point { x: 5, y: 3.5 };
    let p2 = Point {
        x: 'a',
        y: String::from("this is bad"),
    };
    println!("{} {}", p1.get_x(), p1.get_y_f32());

    // p1 and p2 moved to p3, no longer in scope
    let p3 = Point { x: p1, y: p2 };
    // Cannot call get_y_f32 on p3 because it does not exist for y with type of Point
    println!("{p3:?} of type {}", p3.get_type());

    let p1 = Point { x: 5, y: 3.5 };
    let p2 = Point {
        x: 'a',
        y: String::from("this is bad"),
    };
    let p3 = p1.mixup(p2);
    println!("{p3:?}");
}

// Use any Type T that has partial ord trait implemented
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
