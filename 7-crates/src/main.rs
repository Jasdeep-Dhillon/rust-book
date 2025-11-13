// Using crate root to get Vegetables module using absolute path
use crates::garden::vegetables::Asparagus;
// Using package name to access lib module
// Package name for this project = crates
// use crates::front_of_house;
use crates::eat_at_restaurant;
use std::collections::HashMap as HMap;

// Binary Crate because it has main
fn main() {
    let plant = Asparagus::default();
    println!("{plant:?}");
    println!("Plant's weight {}", plant.get_weight());

    eat_at_restaurant();
    
    let mut map = HMap::new();
    map.insert(1,2);
}
