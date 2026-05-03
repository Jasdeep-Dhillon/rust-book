use std::io::Write;

fn main() {
    let mut lyrics = String::new();
    let gifts = [
        "",
        "Two turtle doves,\n",
        "Three French hens,\n",
        "Four calling birds,\n",
        "Five gold rings,\n",
        "Six geese a-laying,\n",
        "Seven swans a-swimming,\n",
        "Eight maids a-milking,\n",
        "Nine ladies dancing,\n",
        "Ten lords a-leaping,\n",
        "Eleven pipers piping,\n",
        "Twelve drummers drumming,\n",
    ];
    for day in 1..=gifts.len() {
        println!(
            "On the {} day of Christmas,",
            match day {
                1 => "first",
                2 => "second",
                3 => "third",
                4 => "fourth",
                5 => "fifth",
                6 => "sixth",
                7 => "seventh",
                8 => "eighth",
                9 => "ninth",
                10 => "tenth",
                11 => "eleventh",
                12 => "twelfth",
                _ => "",
            }
        );
        println!("my true love sent to me");

        if day == 1 {
            println!("A patridge in a pear tree\n");
            continue;
        };
        lyrics.push_str(gifts[day - 1]);
        print!("{}", lyrics);
        std::io::stdout().flush().expect("Failed to flush stdout");
        println!("And a patridge in a pear tree\n");
    }
}
