pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use std::{time, u64};

    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Panic in 'another' test function");
    // }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn overflow_add() {
        add_two(u64::MAX);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {result}"
        );
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    #[ignore]
    fn expensive() {
        let current_time = time::Instant::now();
        std::thread::sleep(time::Duration::from_mins(1));
        let result = current_time.checked_add(time::Duration::from_mins(1)).expect("Panic when trying to add 1 min");
        assert_eq!(time::Instant::now(), result);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
