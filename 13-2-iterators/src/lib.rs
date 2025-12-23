#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_demo() {
        let list = vec![1, 2, 3];

        // Iterators need to be mutable
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let list = vec![1, 2, 3];
        let iter = list.iter();
        // methods that call next() are called consuming adapters
        // sum() consumes/takes ownership of iterator
        let total: i32 = iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adapter() {
        // Iterator adapter modify the result from original iterator while returning a new iterator
        let v = vec![1, 2, 3];
        let adapter: Vec<bool> = v.iter().map(|x| x % 2 == 1).collect();

        assert_eq!(adapter, vec![true, false, true]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
