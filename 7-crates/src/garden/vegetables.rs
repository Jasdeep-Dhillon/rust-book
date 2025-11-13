// Public struct to access it main
#[derive(Debug)]
pub struct Asparagus {
    // Private variable to limit access
    weight: u32,
}

impl Default for Asparagus {
    fn default() -> Self {
        Asparagus { weight: 32 }
    }
}

impl Asparagus {
    // Creating a function to access weight
    pub fn get_weight(&self) -> u32 {
        self.weight
    }
}
