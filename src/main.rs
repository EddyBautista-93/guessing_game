//We need to bring out the input / output libary into scope which comes from the standard libary know as std:
use std::io;
use rand::Rng;

// main function is the entry point into the program.
// the fn syntax declares a new function.
fn main() {
    println!("Guess the number");
    println!("Please input your guess");

    // Secret number
    let secret_number = rand::thread_rng().gen_range(1..10);
// variables are immutable by default, adding mut makes it mutable.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("The Secret Number was : {}", secret_number);

    println!("You guessed: {}", guess);
}
