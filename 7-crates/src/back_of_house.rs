pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
    pub fn get_seasonal_fruit(&self) -> &String {
        &self.seasonal_fruit
    }
}

pub enum Appetizer {
    // All variants of enum are also public
    Soup,
    Salad,
}
