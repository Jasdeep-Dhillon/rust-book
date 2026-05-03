fn main() {
    // let config_max: Option<u8> = Some(3);
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {max}")
    }

    let coins = [
        Coin::Nickel,
        Coin::Dime,
        Coin::Penny,
        Coin::Quarter(UsState::Arizona),
        Coin::Quarter(UsState::California),
        Coin::Quarter(UsState::NewYork),
        Coin::Quarter(UsState::Ohio),
    ];
    // let coin = Coin::Quarter(UsState::California);
    let mut count = 0;
    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            count += 1;
            if let UsState::California = state {
                println!("Quarter from california")
            } 
            if let Some(description)= describe_quarter(coin){
                
            println!("{description}");
            }
        } else {
            println!("Not a quarter");
        }
    }
    println!("Number of quarters: {count}");
}

#[derive(Debug)]
enum UsState {
    Arizona,
    California,
    NewYork,
    Ohio,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Arizona => year >= 1819,
            UsState::California => year >= 1850,
            UsState::NewYork => year >= 1788,
            UsState::Ohio => year >= 1803,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}