use std::io::{self, Write};

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let mut celsius_input = String::new();
    let celsius_value: f64;

    loop {
        print!("Enter the temperature in Celsius: ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is displayed immediately

        celsius_input.clear(); // Clears the string for new input

        io::stdin()
            .read_line(&mut celsius_input)
            .expect("Failed to read line");

        match celsius_input.trim().parse::<f64>() {
            Ok(number) => {
                celsius_value = number;
                break;
            }
            Err(_) => {
                println!("That's not a valid number. Please try again.");
            }
        }
    }

    let fahrenheit = celsius_to_fahrenheit(celsius_value);

    println!("
Temperature in Celsius: {}°C", celsius_value);
    println!("Temperature in Fahrenheit: {}°F", fahrenheit);
}
