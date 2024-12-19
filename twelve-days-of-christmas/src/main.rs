use std::io::Write;
use std::thread;
use std::time::Duration;

// The Gifts are a given. In this array they are ordered and represent that gift on the index-th day.
static GIFTS: [&str; 12] = [
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

static CARDINAL: [&str; 12] = [
    "And A", "Two", "Three", "Four", "FIVE", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
    "Twelve",
];

static ORDINAL: [&str; 12] = [
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth",
    "Eleventh", "Twelfth",
];

fn main() {
    // Making this one based to avoid confusion while enumerating
    for day in 0..12 {
        let day_of_christmas = ORDINAL[day];
        println!("On the {day_of_christmas} of Christmas my true love gave to me...\n");
        thread::sleep(Duration::from_secs(2));
        if day > 0 {
            verse_r(day);
        } else {
            println!("A {}!", GIFTS[0]);
        }
        println!("\n");
        thread::sleep(Duration::from_secs(3));
        clear_console();
    }
}

fn verse_r(day: usize) {
    if day == 4 {
        println!(
            "\n{} {}\n",
            CARDINAL[day].to_uppercase(),
            GIFTS[day - 1].to_uppercase()
        );
        thread::sleep(Duration::from_secs(1));
    } else if day == 0 {
        println!("And a {}!", GIFTS[day - 1]);
    } else {
        println!("{} {}", CARDINAL[day], GIFTS[day]);
    }
    thread::sleep(Duration::from_secs(1));
    if day > 0 {
        verse_r(day - 1);
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap(); // Ensure the output is flushed
}
