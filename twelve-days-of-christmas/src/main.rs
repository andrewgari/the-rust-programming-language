use std::thread;
use std::time::Duration;
use std::io::{Write};

// The Gifts are a given. In this array they are ordered and represent that gift on the index-th day.
static GIFTS: [&str; 12]  = [
    "Partridge in a pear tree",
    "Turtle Doves",
    "French Hens",
    "Calling Birds",
    "Golden Rings",
    "Geese a-laying",
    "Swans a-swimming",
    "Maids a-milking",
    "Pipers piping",
    "Drummers Drumming",
    "Lords a-leaping",
    "Ladies Dancing",
];

fn main() {
    // Making this one based to avoid confusion while enumerating
    for day in 1..13 {
        let day_of_christmas = ordinal(day);
        println!("On the {day_of_christmas} of Christmas my true love gave to me...\n");
        thread::sleep(Duration::from_secs(2));
        if day > 1 {
            verse_r(day as usize);
        } else {
            println!("A {}!", GIFTS[0].to_string());
        }
        println!("\n");
        thread::sleep(Duration::from_secs(3));
        clear_console();
    }
}

// Converts any number to it's ordinal.
fn ordinal(n: u32) -> String {
    match n {
        1 => "First".to_string(),
        2 => "Second".to_string(),
        3 => "Third".to_string(),
        4 => "Fourth".to_string(),
        5 => "Fifth".to_string(),
        6 => "Sixth".to_string(),
        7 => "Seventh".to_string(),
        8 => "Eighth".to_string(),
        9 => "Ninth".to_string(),
        10 => "Tenth".to_string(),
        11 => "Eleventh".to_string(),
        12 => "Twelfth".to_string(),
        _ => n.to_string(), // default case
    }
}

// Converts the number to it's cardinal, however, "One" is replaced with "And A" to better fit the song.
// If it's the First day of Christmas, the First Gift is not enumerated ans is simply "A"jk;j
fn cardinal(n: usize) -> String {
    match n {
        1 => "And a".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        11 => "Eleven".to_string(),
        12 => "Twelve".to_string(),
        _ => n.to_string(), // default case
    }
}

fn verse_r(day: usize) {
    if day > 0 {
        if day == 5 {
            println!("\n{} {}\n", cardinal(day).to_uppercase(), GIFTS[day - 1].to_string().to_uppercase());
            thread::sleep(Duration::from_secs(1));
        } else if day == 1 {
            println!("And a {}!", GIFTS[day - 1].to_string());
        } else {
            println!("{} {}", cardinal(day), GIFTS[day - 1].to_string());
        }
        thread::sleep(Duration::from_secs(1));
        verse_r(day-1);
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap(); // Ensure the output is flushed
}