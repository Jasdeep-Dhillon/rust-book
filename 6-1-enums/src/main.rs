enum IpAddrKind {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}
impl IpAddrKind {
    // Printing the value of the IpAddr
    fn print_value(&self) {
        // Using match to differentiate between the two types
        match self {
            Self::V4(a, b, c, d) => {
                println!("{a}.{b}.{c}.{d}");
            }
            Self::V6(addr) => {
                println!("{addr}");
            }
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        match self {
            Self::Quit => {
                println!("Quitting");
            }
            Self::Move { x, y } => {
                println!("Moving {x} horizontally {y} vertically ");
            }
            Self::Write(message) => {
                println!("Writing {message}");
            }
            Self::ChangeColor(r, g, b) => {
                println!("Changing Color to {r} {g} {b}");
            }
        }
    }
    // No function overloading
    // fn call() {
    //     println!("Message call global");
    // }
}

fn main() {
    // Instantiating IppAddrKind enum
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    // Calling method on self
    four.print_value();
    six.print_value();

    let m = Message::Write(String::from("Hello Enum"));
    let q = Message::Quit;
    let mmove = Message::Move { x: 2, y: 2 };
    let color = Message::ChangeColor(0, 0, 0);
    mmove.call();
    color.call();
    m.call();
    q.call();

    // Option Enum
    let _number = Some(5);
    let _char = Some('e');
    // Needs explicit type definition, because None has no type information
    let _none: Option<i32> = None;
}
