use rand::Rng; // Rng is a trait (?)
use std::cmp::Ordering;
use std::io; // used for getting user input from terminal and output

fn main() {
    println!("Guess the number!"); // println! is a macro

    let secret_number = rand::thread_rng().gen_range(1..=10);
   // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // `let` creates a variable. `mut` means its value can change
        // variables are inmutable by default.
        // `String::new` returns a new instance of a String
        // `::` means `new` is a function associated to the type
        let mut guess = String::new();

        // references are inmutable by default too
        // read_line returns a `Result`, used for error check
        // if error then print the message
        io::stdin()
            .read_line(&mut guess) // `&` means a reference
            .expect("Failed to read line");

        // shadowing `guess` to convert it to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); // placeholder

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
