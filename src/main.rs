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
    // whitelist of operators, not listed is not accepted.
    let operators = String::from("+-*/");

	// clear the variables
	num1.clear();
	num2.clear();
	operator.clear();

	/* TODO:
	* make function for user input
	* get valid number input else give a loop to repeat or quit.
	* applicable for nums 1 and 2
	* for operator variable still a todo.

	*/
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


    // conditionals with negation
    if !operators.contains(operator) {
        // loop is not working, need to make a function for looping
        println!("Unknown operator, use valid symbols [ + - * / ]");
    }
    // note the use of single quote inside match block since it is matching string literals
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("error in operator"),
    }; // note the closing semicolon
       // show the operation output
    println!("the result of {} {} {} = {}", num1, operator, num2, result);
}
