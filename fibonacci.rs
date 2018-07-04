use std::io::{Write, stdin, stdout};

fn main() {
    const round: bool = true;
    let mut selection = String::new();

    print!("What number of fibonacci would you like to go to?\n>> ");
    stdout().flush();
    stdin().read_line(&mut selection);

    let selection = selection.trim().parse::<f32>().unwrap();

    if round {println!("{}", fibonacci(selection).round())}
    else {println!("{}", fibonacci(selection))}
}

fn fibonacci(n: f32) -> f32 {(f32::powf(1.6180339, n as f32) - f32::powf(-0.6180339, n as f32)) / (2.236067977)}
