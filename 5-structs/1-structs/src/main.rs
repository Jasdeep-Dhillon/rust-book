// Defining the User struct
struct User {
    active: bool,
    sign_in_count: u64,
    // Not using string literals because we need to specify lifetimes
    username: String,
    email: String,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Struct
struct UnitStruct;

fn main() {
    // Creating User object
    // Must be mutable to edit fields
    let mut user1 = User {
        active: true,
        username: String::from("Arc"),
        email: String::from("arc@realmail.com"),
        sign_in_count: 2,
    };
    user1.email = String::from("arc@mail.com");
    println!(
        "Active: {}\nUsername:{}\nEmail:{}\nSign In Count:{}\n",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let _user2 = build_user(String::from("arc2@mail.com"), String::from("arc2"));

    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@arc.ca"),
    //     sign_in_count: user1.sign_in_count
    // };
    let _user3 = User {
        email: String::from("another@arc.ca"),
        // Must be last / Cannot have comma after
        // Base struct must be last field
        ..user1 // User 1 username transferred ownership
                // Sign in and active implement copy trait they are copied instead of being transferred
    };

    println!("{}\n", user1.email);
    // Throws error
    // println!("{}", user1.username);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    // Doesn't work because the type is different despite having same data
    // let origin: Color = Point(0, 0, 0);
    // Destructing tuple structs
    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;
    println!("Color: {},{},{}\n", r, g, b);
    println!("Point: {},{},{}\n", x, y, z);
    // Destructing struct
    let User {
        email,
        username,
        active,
        sign_in_count,
    } = _user2;
    println!(
        "Email: {}\nUsername:{}\nActive:{}\nSign Count:{}",
        email, username, active, sign_in_count
    );

    // Initializing unit structs
    let _unit = UnitStruct;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Init shorthand, field name and variable name is the same
        username,
        email,
        sign_in_count: 1,
    }
}
