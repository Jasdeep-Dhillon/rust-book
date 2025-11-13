// Library crate no main can only call functions or import modules
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house;
pub use front_of_house::{hosting, serving};

mod back_of_house;

pub mod garden;
use garden::vegetables::Asparagus;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative Path
    hosting::add_to_waitlist();
    
    serving::serve_order();
    serving::take_payment();

    let _asparagus = Asparagus::default();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!(
        "I'd like {} toast please with {}",
        meal.toast,
        meal.get_seasonal_fruit()
    );

    // Cannot edit private variable
    // meal.seasonal_fruit = String::from("strawberries");
    // let _ = back_of_house::Breakfast {
    //     toast: String::from("Wheat"),
    //     // Private field cannot assign value without a pub method
    //     seasonal_fruit: String::from("strawberries")
    // };

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
