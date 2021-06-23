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
	
	// make the loop
	
	loop {

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
    
    	// do some initial debug
    	// println!("num1: {} num2 {} operator: {}", num1, num2, operator);
    	
		// whitelist of operators, not listed is not accepted.
    	let operators = String::from("+-*/");
    	
    	// conditionals with negation
    	if !operators.contains(operator) {
			// loop is not working, need to make a function for looping
			println!("Unknown operator, use valid symbols [ + - * / ]");
    	    //return;
    	    // use continue to loop over
			continue;
    	}
    	// note the use of single quote inside match block since it is matching string literals
    	let result = match operator {
    	    '+' => num1 + num2,
    	    '-' => num1 - num2,
    	    '*' => num1 * num2,
    	    '/' => num1 / num2,
    	    _ => panic!("error in operator")
    	};      // note the closing semicolon
    	// show the operation output
    	println!("the result of {} {} {} = {}", num1, operator, num2, result);
	}
}
