// guppies_basic
// 
// Author: Harsh Patel, Mark Liffiton
// Date: November, 2022
//
// A basic implementation of the Guppies game in Rust.
//

// FYI, Rust comments starting with /// are "doc comments," specifically for building
// documentation.  Regular comments (not for building documentation, but just to be read in the
// code itself, start with //.

use std::io;  // for reading from stdin
use rand::Rng;  // for generating random numbers
use colored::*;  // for coloring printed output

/// Prints a given prompt and reads a line of input from stdin as a String.
///
/// # Arguments
///
/// * `prompt` - A string slice that holds the prompt to be printed.
///
fn read_input(prompt: &str) -> String {
    println!("{}", prompt.yellow());
    let mut line = String::new();  // buffer for reading input from the user
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let trimmed = line.trim();  // drop whitespace
    return trimmed.to_string();
}

/// Prints a given prompt and reads an integer from stdin as an i32.
/// Prints an error and requests input again as long as the user enters something
/// other than an integer.
///
/// # Arguments
///
/// * `prompt` - A string slice that holds the prompt to be printed.
///
fn read_int_input(prompt: &str) -> i32 {
    loop {
        let line = read_input(prompt);
        let parsed = line.parse::<i32>();
        match parsed {
            Ok(i) => return i,
            Err(..) => println!("{}  Try again...", "That's not an integer.".red()),
        }
    }
}

/// Generates and returns a random number between 1 and 10, inclusive.
fn get_random() -> i32 {
    rand::thread_rng().gen_range(1..11)
}

/// Prompts the user for a guess in standard Guppies, returning the guess as a String.
/// Prints an error and requests input again as long as the user enters an invalid guess.
fn get_guess() -> String {
    let mut guess = read_input("Is the second number (h)igher, (l)ower, or the (s)ame?   [Or (q)uit.]");
    guess = guess.to_lowercase();
    while guess != "h" && guess != "l" && guess != "s" && guess != "q" {
        guess = read_input("Invalid guess.  Is the second number (h)igher, (l)ower, or the (s)ame?   [Or (q)uit.]");
        guess = guess.to_lowercase();
    }
    return guess;
}

/// Checks a guess against the first and second generated numbers in standard Guppies.
///
/// # Arguments
///
/// * `first` - The first generated number.
/// * `second` - The second generated number.
/// * `guess` - A string slice holding the user's guess.
///
/// # Returns
///
/// * bool: True if the guess was correct, False otherwise.
///
fn check_guess(first: i32, second: i32, guess: &str) -> bool {
    match guess.to_lowercase().as_str() {
        "h" => second > first,
        "l" => second < first,
        "s" => second == first,
        _ => panic!("Ooh, bad guess...")
    }
    // Equivalent to:
    //  if guess == "h" && second > first || guess == "l" && second < first || guess == "s" && second == first
    // but this is cleaner, easier to read, and has bonus error-checking (though just a crash in
    // this case, which isn't ideal).
}

fn main() {
    println!("{}", "Welcome to Guppies!".bright_purple());

    let mut money = 100;

    while money > 0 {
        // Announce the current "score."
        println!("You currently have {} bucks.", money);
        
        // Get a bet from the user.
        let mut bet = read_int_input("What is your bet?");
        while bet < 0 || bet > money {
            println!("{}  Try again...", "Invalid bet.".red());
            bet = read_int_input("What is your bet?");
        }

        // Generate and show the first random number from 1 to 10 inclusive.
        let first = get_random();
        println!("The first number is: {}", first);

        // Get the user's guess.
        let guess = get_guess();

        if guess == "q" {
            break;
        }

        // Generate and show the second number.
        let second = get_random();
        println!("The second number is: {}", second);

        // Check the player's guess and award/remove their bet as appropriate.
        let result = check_guess(first, second, &guess);
        if result {
            println!("You were right!");
            money += bet;
        }
        else {
            println!("You were incorrect.");
            money -= bet;
        }
    }

    // We got here one of two ways: either the user ran out of money,
    // or else the user chose to quit.  Report the result in either case.
    if money == 0 {
        println!("{}", "You're broke. :-/".red());
    }
    else {
        println!("{}", "You made it out!".bright_green());
        println!("You currently have {} bucks.", money);
    }
}
