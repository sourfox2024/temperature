use std::io::{self, Write};

fn main() {
    let celsius_value = get_temperature_from_user();
    let fahrenheit = celsius_to_fahrenheit(celsius_value);

    println!("
Temperature in Celsius: {}°C", celsius_value);
    println!("Temperature in Fahrenheit: {}°F", fahrenheit);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn get_temperature_from_user() -> f64 {
    let mut input = String::new();
    loop {
        print!("Enter the temperature in Celsius: ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => {
                println!("That's not a valid number. Please try again.");
            }
        }
    }
}
