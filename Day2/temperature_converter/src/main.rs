// Import input/output library
use std::io;

// Main function
fn main() {
    // Menu
    println!("Temperature Converter");
    println!("1- Celsius to Fahrenheit");
    println!("2- Fahrenheit to Celsius");
    println!("Please select an option (1 or 2):");

    // Save user response
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    // Validate user response
    let choice:u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice. Please enter 1 or 2");
            return;
        }
    };

    // Choose action to execute
    if choice == 1 {
        celsius_to_farenheit()
    } else if choice == 2 {
        farenheit_to_celsius()
    } else {
        println!("Invalid choice. Please enter 1 or 2");
    }
}

/*
Function to convert celsius to farenheit
*/
fn celsius_to_farenheit() {
    let celsius = get_temperature("Celsius");
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{:.2}º Celsius is {:.2}º Fahrenheit", celsius, fahrenheit);
}

/*
Function to convert celsius to farenheit
*/
fn farenheit_to_celsius() {
    let fahrenheit = get_temperature("Fahrenheit");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}º Fahrenheit is {:.2}º Celsius", fahrenheit, celsius);
}

/*
Function to obtain the original temperature from console
*/
fn get_temperature(cadena:&str) -> f64 {
    println!("Enter temperature in {}: ", cadena);
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp:f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature. Please enter a number.");
            return 0.0;
        }
    };

    return temp;
}