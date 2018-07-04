// Imports
use std::io::{Write, stdin, stdout};

// Main Function
fn main() {
    // Predefine variables
    let mut selection = String::new();
    let mut number = String::new();


    // Print and get input
    print!("Conversion Methods:\n[1] Celsius to Fahrenheit\n[2] Fahrenheit to Celsius\n>> ");
    stdout().flush();
    stdin().read_line(&mut selection);

    print!("Enter a number to convert: ");
    stdout().flush();
    stdin().read_line(&mut number);

    // Convert strings to integers and floats
    let selection = selection.trim().parse::<i32>().unwrap();
    let number = number.trim().parse::<f32>().unwrap();

    // Check what selection was made
    if selection == 1 {
        println!("{}°f", to_fahrenheit(number));
    } else if selection == 2 {
        println!("{}°c", to_celsius(number));
    } else {
        eprintln!("Not a valid method!");
    }
}

// Functions to convert temperature
fn to_fahrenheit(number: f32) -> f32 {number * 1.8 + 32.0}
fn to_celsius(number: f32) -> f32 {(number - 32.0) * 0.5556}
