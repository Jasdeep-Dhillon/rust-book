pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand;
    #[test]
    fn it_works() {
        let num = rand::random_range(1..=100);
        assert_eq!(num + 1, add_one(num));
    }
}
