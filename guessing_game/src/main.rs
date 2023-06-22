use std::io; // used for getting user input from terminal and output

fn main() {
    println!("Guess the number!"); // println! is a macro
    
    println!("Please input your guess.");
    
    // let creates a variable. mut means its value can change
    // variables are inmutable by default
    // `String::new` returns a new instance of a String
    // `::` means `new` is a function associated to the type
    let mut guess = String::new();

    // references are inmutable by default too
    // read_line returns a `Result`, used for error check
    // if error then print the message
    io::stdin()
        .read_line(&mut guess) // & means a reference
        .expect("Failed to read line");

    println!("You guessed: {guess}"); // placeholder
}
