// bring the io input/output library into scope
// needed to obtain user input and then print the
// result as an output
use std::io;

// this file, this program we are doing is a binary crate
// ehich is an executable
// the rand crate (which we use to get a rand num for our game here) is considered a library crate
// which contains code inteded to be used in other programs and can not be excuted on its own
//before being able to acces rand library, modify Cargo.toml to add it as a dependency

// entry point of program
fn main() {
    // a macro that prints data to the screen
    println!("Guess the number!");

    println!("Please input your guess.");

    // variables in rust are by default immuntable
    // add mut to make mutable
    // = tells rust to BIND a value/something
    // String::new() returns a new String type 
    // :: before the new tells us that new is an associate
    // type to String. new creates a new empty string.
    // so it means:
    // line 21 creates a mutable variablethat is currently bound to a new
    // empty instance of a String =>
    let mut guess = String::new();

    // recieve user input
    // stdin is from the std::io library
    // without that library line 28 would be:
    // std::io::stdin
    io::stdin()
        // & sign is saying its a reference to that variable, so it is not copied 
        // again to memory
        // read_line is from the std::io library - takes user input and appends it to guess (without overwriting its contents)
        .read_line(&mut guess)

        // handle failure with result
        // read_line puts whatever the user enters into the string we pass to it,
        // but it also returns a Result value. Result is an enumeration, often called an enum, which is
        // a type that can be in one of multiple possible states. We call each possible state a variant
        // Result ’s variants are Ok and Err
        // expect method is from the current Result value instance
        .expect("Failed to read line!");

    // {} placeholder
    // 
    println!("You guessed: {guess}");
}
