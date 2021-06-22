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

	print!("what is the first number?: ");
	read(&mut num1);

	print!("what is the second number?: ");
	read(&mut num2);

	print!("what operation would you like to do? [+-*/]: ");
	read(&mut operator);

	// change input to numbers from strings
	let num1: f32 = num1.trim().parse().unwrap();
	let num2: f32 = num2.trim().parse().unwrap();
	let operator: char = operator.trim().chars().next().unwrap();
}
