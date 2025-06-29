use std::io::{self, Write};

fn main() {
    // Let's variable 'temperature' become mutable
    let mut celcius = String::new();

    loop {
        print!("Enter the temperature in celcius: ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is displayed immediately

        celcius.clear(); //clears the string for new input.

        io::stdin()
            .read_line(&mut celcius)
            .expect("Failed to read line");

        if let Ok(number) = celcius.trim().parse::<f64>() {
            break;
        } else {
            println!("That's not a valid number. Please try again.")
        }
    }

    // convert celsius string to integer c
    match celcius.trim().parse::<i32>() {
        Ok(input_num) => {
            let c = input_num;
            let fahrenheit = c as f64 * 9.0 / 5.0 + 32.0;

            println!("\nTemperature in Celsius: {}°C", c);
            println!("Temperature in Fahrenheit: {}°F", fahrenheit)
        }
        Err(error) => println!("converting error: {error}"),
    }
}
