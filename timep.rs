// timep.rs
// Use this program to time programs.
// For windows only, but if you're in linux just use "time"

use std::time::Instant;
use std::process::Command;
use std::env;
use std::str;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.drain(0..1); // Remove first element
	let joined_args = args.join(" ");
	
	let now = Instant::now();
	// Do command
	let output = Command::new("cmd")
			.args(&["/C", joined_args.as_str()])
			.output()
			.expect("Failed to time process");

	let elapsed = now.elapsed();
	println!("{}", str::from_utf8(&output.stdout).unwrap());
	println!("{:?}", elapsed);
}
