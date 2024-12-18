use std::io;

fn main() {
    println!("Please enter a temperature in degrees Farenheight...");
    let mut temp = String::new();
    loop {
        io::stdin()
            .read_line(&mut temp)
            .expect("You BLEW ITTTTTTTTTTTTTTTTTTTT");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                warn!("Please enter a valid number, idiot");
                continue;
            },
        };
        // c = (f - 32) * 5/9
        let temp: f32 = convert_to_celsius(temp);
        println!("You printed {temp}");
        break;
    }
}

fn convert_to_celsius(degrees_f: f32) -> f32 {
    return (degrees_f - 32.0) * 5.0/9.0;
}
