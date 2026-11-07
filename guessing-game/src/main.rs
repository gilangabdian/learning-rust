// this is the main code of guessing-game that made by using cargo new
// Every time you use cargo new to create rust project, it will automatically create
// `src` and target directory (in `src`, it will an example Hello world program), and also files `Cargo.toml` and `Cargo.lock`

// ----------------------------------------------------------------------------

// We need to use `io` input/output library
// The library is comes from the standart library, also know as `std`
use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

// main function is always be entry point for the program
fn main() {

  // Use `println!` is known as macro, it will make the world appear into the screen
  println!("Guess the number!");

  let secret_number = rand::rng().random_range(1..=100);

  println!("The secret number is: {secret_number}");
   loop {
        println!("Please input your guess.");


  // Variable on rust by default is immutable
  // If we want the variable is mutable, we must add `mud` before variable name
  // This line of code, means that we want introduce a mutable variable named guess
  // Variable `guess` have a function that return a new empty instance of String as a value
  let mut guess = String::new();


  // `stdin` is a function from `io` module that we import in top of this file
  // The `stdin` function will return an instance of `std::io::Stdin`
  io::stdin()

  // The `&` in this argument indicates as a references
  // It will let the code to access one piece of data without always copy that data into memory multiply times
  // Reference by default also immutable, so for make it mutable we must add 'mut' too
    .read_line(&mut guess)

    // Cause `read_line` is also return `Results` value, it means `enumeration`
    // That usually use to handle error information
    // `Result` value can have instance, the instance of `Result` value have `expect` method that we can call
    // If we dont call the `expect` method, the program will compile, but you will get a warning
    .expect("Failed to read line");


    // This line of codes for print the result and contains the user input
    // The `{}` is a placeholder
    // Printing a variable and the result of an expression in one call whould look likt this:
    // println!("x = {} and y = {}", x, y)
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
  println!("You guessed: {guess}");

  match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
    };
   }
}
