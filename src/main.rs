// simple calculator app in Rust

use std::io::{stdin, stdout, Write};

// create a function
fn read(input: &mut String) {
	stdout().flush().expect("failed to flush");
	stdin().read_line(input).expect("failed to read");
}
fn main() {
    println!("Welcome to Rust simple calculator program!");
	println!("------");

	let mut num1 = String::new();
	let mut num2 = String::new();
	let mut operator = String::new();

	println!("what is the first number?: ");
}