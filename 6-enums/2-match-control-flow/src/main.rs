#[derive(Debug)]
enum UsState {
    Arizona,
    California,
    NewYork,
    Ohio,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn cent_value(self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // Match takes ownership of the variable
                // Passing reference to keep ownership
                match &state {
                    UsState::California => println!("Quarter from California"),
                    // Unmatched values default to this pattern
                    default => println!("Quarter from {default:?}"),
                }
                // println!("State quarter from {state:?}");
                dbg!(&state);
                25
            }
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Arizona);
    coin.cent_value();

    plus_one(None);
    plus_one(Some(300));
    // Errors out because of buffer overflow
    // plus_one(Some(i32::MAX));

    let dice_roll: u8 = rand::random_range(1..=10);
    print!("Dice: {dice_roll}\n");
    match dice_roll {
        3 => println!("Received hat"),
        7 => println!("Removed hat"),
        _ => {
            if dice_roll % 2 == 0 {
                println!("Even roll");
            } else {
                println!("Odd roll");
            }
        }
    }
    let _ = [
        Coin::Nickel,
        Coin::Dime,
        Coin::Penny,
        Coin::Quarter(UsState::Arizona),
        Coin::Quarter(UsState::California),
        Coin::Quarter(UsState::NewYork),
        Coin::Quarter(UsState::Ohio),
    ];
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            print!("Value is {i}\n");
            Some(i + 1)
        }
    }
}
