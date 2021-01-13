use std::io::{self, Write};
use argon2::{self, Config};

fn main() {
	encrypt();
	check();
}

fn encrypt() {
	let password = input("Password: ");
	let password = password.as_bytes();

	let salt = input("Salt: ");
	let salt = salt.as_bytes();

	let config = Config::default();
	let hash = argon2::hash_encoded(password, salt, &config).unwrap();

	println!("Hash: {:?}", &hash);
}

fn check() {
	println!("[Password Checker]");
	let new_hash = input("Hash: ");
	let new_hash = new_hash;
	let new_password = input("Password Guess: ");
	let new_password = new_password.as_bytes();

	let new_match = argon2::verify_encoded(&new_hash, new_password).unwrap();

	if new_match {
		println!("Password was correct.");
	} else {
		println!("Password was incorrect. Match is {:?}", new_match);
	}
}

fn input(message: &str) -> String {
	let mut input_msg = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut input_msg)
		.expect("Failed to get input.");

	let input_msg = input_msg.trim();
	let input_msg = input_msg.to_owned();

	input_msg
}