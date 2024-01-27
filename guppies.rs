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

use core::panic;
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


/// Trait that allows the caller of run_game() to pass in an object that modifies the currency
/// 
/// # Methods
/// 
/// * starting_amount is the starting amount of money they have
/// * print_amount prints the amount of money they have
///
trait Currency{
    // Starting amount, just returns an i32
    fn starting_amount(&self) -> i32;
    // takes an argument amount which is an i32 and returns a string
    fn print_amount(&self, amount:i32);
}


// Dollar struct which implements the currency trait
struct Dollar{}
impl Currency for Dollar {
    // prints the starting amount
    fn starting_amount(&self) -> i32 {
        100
    }
    // Method prints the amout of Dollars the user has
    fn print_amount(&self, amount:i32){
        println!("You currecntly have {} Dollars", amount)
    }
}


// TurkishLira struct which implements the currency trait
struct TurkishLira{}
impl Currency for TurkishLira{
    fn starting_amount(&self) -> i32 {
        100000000
    }
    // Method prints the amout of TurkishLira the user has
    fn print_amount(&self, amount:i32){
        println!("You currecntly have {} Turkish Lira", amount)
    }
}

// Hbuck struct which implements the currency trait
struct Hbuck{}
impl Currency for Hbuck{
    fn starting_amount(&self) -> i32 {
        32199
    }
    // Method prints the amout of Hbucks the user has
    fn print_amount(&self, amount:i32) {
        println!("You currecntly have {} Hbucks", amount)
    }
}

// Different values is an enum which has two different values
enum DifferentValues {
    FirstGeneratedVal,
    SecondGeneratedVal,
}


/// GuppiesVariant is a trait,which are different variants of gameplay
///
/// # Methods
/// 
/// * 'generate_new_random' is a mutable reference that genereats a random value_one
/// * 'tell_random' tells the user the two randomly generated values, takes an enum as an argument
/// that tells the user (prints out) one of the two generated values.  
/// It takes an enum (variant) type that lets the method's caller choose whether to print 
/// the first generated value or the second generated value. 
/// * 'get_guess' gets the guess from the user and returns a string
/// * 'check_guess' takes a guess as an argument which is a string and returns a bool 
/// depending on if the guess was correct or not
trait GuppiesVariant {
    fn generate_new_random(&mut self);
    fn tell_random(&self, value: DifferentValues);
    fn get_guess(&self) -> String;
    fn check_guess(&self, guess:&str) ->bool;
}

// Struct OddOrEvenGuppies is a game which asks the user if the number is odd or even
// Two constructors, num_one and num_two which are of type i32
struct OddOrEvenGuppies {
    num_one: i32,
    num_two: i32,
}

// This struct implements GuppiesVariant
impl GuppiesVariant for OddOrEvenGuppies {
    fn generate_new_random(&mut self) {
        // Genereates two random numbers and assigns them to the 
        // Constructos of the struct
        self.num_one = rand::thread_rng().gen_range(1..101);
        self.num_two = rand::thread_rng().gen_range(1..101);
    }

    fn tell_random(&self, value:DifferentValues) {
        // Uses patternmatching to use the enum
        match value {
            DifferentValues::FirstGeneratedVal => {
              println!("This is the first number {}", self.num_one) 
            }
            DifferentValues::SecondGeneratedVal => {
                println!("This is the second number {}",self.num_two) 
              }
            }
        }
              
    // Gets the guess from the user and makes sure its not invalid, returns a guess as
    fn get_guess(&self) -> String {
        let mut guess = read_input("Is the second num (o)dd, (e)ven, or the (s)ame? [Or (q)uit.]");
        guess = guess.to_lowercase();
        while guess != "o" && guess != "e" && guess != "s" && guess != "q" {
            guess = read_input("Invalid guess. Is the second num (o)dd, (e)ven, or the (s)ame? [Or (q)uit.]");
            guess = guess.to_lowercase();
        }
    return guess;
    }

    // Checks the users guesses
    fn check_guess(&self, guess:&str) -> bool {
       match guess.to_lowercase().as_str() {
        // odd if there is a remainder
        "o" => self.num_two % 2 != 0, 
        // even if there is no remainder
        "e" => self.num_two % 2 == 0,
        "s" => self.num_two == self.num_one,
        _ => panic!("Ooh, bad guess...")
       }
    }
 } 


// RainbowGuppies variant of gameplay, which lists colors and asks the user if first color
// is farther or closer to green when compared with the second color
struct RainbowGuppies {
    color_one: String,
    color_two: String,
}

impl GuppiesVariant for RainbowGuppies {
    fn generate_new_random(&mut self) {
        // Vector to store the colors
        let colors = vec!["Violet", "Indigo", "Blue","Green", "Yellow", "Orange", "Red"];
        // generates a random index from the range 0 to the end of the vector
        let index = rand::thread_rng().gen_range(0..colors.len());
        // stores those strings inside the constructors
        self.color_one = colors[index].to_string();
        self.color_two = colors[index].to_string();
    }

    // Outputs the constructors to the user
    fn tell_random(&self, value: DifferentValues) {
        match value {
            DifferentValues::FirstGeneratedVal => {
                println!("This is first color {}", self.color_one)
            }
            DifferentValues::SecondGeneratedVal => {
                println!("This is second color {}", self.color_two)
            }
        }
    }

    // gets the guesses from the user
    fn get_guess(&self) -> String {
        let mut guess = read_input("Is the second color (c)loser, (f)arther, or the (s)ame when compared with green? [Or (q)uit.]");
        guess = guess.to_lowercase();
        while guess != "c" && guess != "f" && guess != "s" && guess != "q" {
            guess = read_input("Invalid guess. Is the second color (c)loser, (f)arther, or the (s)ame when compared with green?? [Or (q)uit.]");
            guess = guess.to_lowercase();
        }
        return guess;
    }

    // Resource: https://stackoverflow.com/questions/30558246/how-do-i-find-the-index-of-an-element-in-an-array-vector-or-slice
    // Checks the validity of the guess
    fn check_guess(&self, guess:&str) -> bool {
        let colors = vec!["Violet", "Indigo", "Blue","Green", "Yellow", "Orange", "Red"];
       
        // Used to obtian the index of green, color_one, and color_two
        let index_one = colors.iter().position(|&r| r == self.color_two).unwrap();
        let index_two = colors.iter().position(|&r| r == self.color_one).unwrap();
        let index_green = colors.iter().position(|&r| r == "Green").unwrap();
        
        match guess.to_lowercase().as_str() {
        // subtracts the colors index with the green index and compares
        // if color one's index - green index is < color twos index - green index then its closer
        // if its >, then its farther  
         "c" => index_one - index_green < index_two - index_green,
         "f" => index_one - index_green > index_two - index_green,
         "s" => self.color_two == self.color_one,
         _ => panic!("Ooh, bad guess...")
        }
     }
}


// Plain guppies is the original variant
struct PlainGuppies {
    value_one: i32,
    value_two: i32,
}

impl GuppiesVariant for PlainGuppies {
    fn generate_new_random(&mut self) {
        // generates two random values and sets them to the constructor
        self.value_one = rand::thread_rng().gen_range(1..11);
        self.value_two = rand::thread_rng().gen_range(1..11);
    }
    // outputs the two values to the user
    fn tell_random(&self, value:DifferentValues) {
        //typename::constructor (construct is a vlaue)
        match value {
            DifferentValues::FirstGeneratedVal => {
              println!("This is the first value {}", self.value_one) 
            }
            DifferentValues::SecondGeneratedVal => {
                println!("This is the second value {}",self.value_two) 
              }
            }
        }
              
    //  Gets the guess from the user
    fn get_guess(&self) -> String {
        let mut guess = read_input("Is the second number (h)igher, (l)ower, or the (s)ame?   [Or (q)uit.]");
        guess = guess.to_lowercase();
        while guess != "h" && guess != "l" && guess != "s" && guess != "q" {
            guess = read_input("Invalid guess.  Is the second number (h)igher, (l)ower, or the (s)ame?   [Or (q)uit.]");
            guess = guess.to_lowercase();
        }
    return guess;
    }

    // checks the validity of the guess
    fn check_guess(&self, guess:&str) -> bool {
       match guess.to_lowercase().as_str() {
        "h" => self.value_two > self.value_one,
        "l" => self.value_two < self.value_one,
        "s" => self.value_two == self.value_one,
        _ => panic!("Ooh, bad guess...")
       }
    }
 }


/// Resource: https://stackoverflow.com/questions/36413364/as-i-can-make-the-vector-is-mutable-inside-struct
/// Manyguppies stores all the variants in a vector and randomly chooses one
struct Manyguppies{
    // This constructor is a vector which stores GuppiesVariants
    guppies_variants : Vec<Box<dyn GuppiesVariant>>,
    // This constructor is box which stores one GuppiesVariant
    current_variant :  Box<dyn GuppiesVariant>,
 }

impl GuppiesVariant for Manyguppies  {
    fn generate_new_random(&mut self) {
        // generates a random index 
        let variant_index = rand::thread_rng().gen_range(0..self.guppies_variants.len());
        // sets the current variant to that random index
        self.current_variant = self.guppies_variants[variant_index];

        self.current_variant.generate_new_random();
    }
    
    // Calls the current variiants implementation of the method
    fn tell_random(&self, value: DifferentValues) {
        self.current_variant.tell_random(value)
    }
    fn get_guess(&self) -> String {
        self.current_variant.get_guess()
        
    }
    fn check_guess(&self, guess:&str) ->bool {
        self.current_variant.check_guess(guess)
    }
 }


/// This function runs the actual gameplay of the guppies game
/// 
/// # Arguments
/// 
/// 
/// * 'currency' is a box which holds the trait Currency
/// * 'variant' is a box which holds the trait GuppiesVariant
fn run_game(currency: Box<dyn Currency>, mut variant: Box<dyn GuppiesVariant>){

    // This variable stores the different currencies starting amount
    let mut money = currency.starting_amount();

    while money > 0 {
        // prints the starting amount of money
        currency.print_amount(money);
        
        // Get a bet from the user.
        let mut bet = read_int_input("What is your bet?");
        while bet < 0 || bet > money {
            println!("{}  Try again...", "Invalid bet.".red());
            bet = read_int_input("What is your bet?");
        }

        // Generates the first random value and tells the user that value
        variant.generate_new_random();
        variant.tell_random(DifferentValues::FirstGeneratedVal);

        // gets the guess 
        let guess = variant.get_guess();

        if guess == "q" {
            break;
        }
        // Generates the second random value and tells the user that value
        variant.generate_new_random();
        variant.tell_random(DifferentValues::SecondGeneratedVal);

        // Check the player's guess and award/remove their bet as appropriate.
        let result = variant.check_guess(&guess);
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


fn main() {
    println!("{}", "Welcome to Guppies!".bright_purple());
    
    // Lets the user choose a currency
    println!("Choose a currency: 1: Dollar, 2: Turkish Lira, 3: Hbucks");

    // prompts the user to choose one of the currency choices
    let currency_choose = read_int_input("Choose the correct corresponding number:");
    // uses pattern matching to assign the users choice to the corresponding Currency
    let currency :Box<dyn Currency> = 
    match currency_choose {
        1 => Box::new(Dollar{}),
        2 => Box::new(TurkishLira{}),
        3 => Box::new(Hbuck{}),
        _ => panic!("Please enter a valid number")
    };

    // prompts the user to choose one of the variant choices
    println!("Choose a GameMode: 1:PlainGuppies, 2:RainbowGuppies, 3:OddOrEvenGuppies , 4:Manyguppies");

    // stores the different variants so i can store them in the vector  
    let plain: Box<PlainGuppies> = Box::new(PlainGuppies {value_one: 1, value_two: 2});
    let rainbow: Box<RainbowGuppies> = Box::new(RainbowGuppies {color_one: "Red".to_string(), color_two: "Blue".to_string()});
    let oddoreven: Box<OddOrEvenGuppies> =  Box::new(OddOrEvenGuppies{num_one : 0, num_two: 0});

    // Stores all the variants inside a vector so i can use it for ManyGuppies
    let many_var: Vec<Box<dyn GuppiesVariant>> = vec![plain, rainbow, oddoreven];
    // stores the current variant, chose a random index to start
    let current_var: Box<dyn GuppiesVariant> = many_var[0];

    // prompts the user to choose a variant
    let variant_choice = read_int_input("Choose the correct corresponding number:");
    
    // uses pattern matching to assign the correct variant to the users choice
    let variant : Box<dyn GuppiesVariant> = 
    match variant_choice {
        1 => Box::new(PlainGuppies {value_one: 1, value_two: 2}),
        2 => Box::new(RainbowGuppies {color_one: "Indigo".to_string(), color_two: "Yellow".to_string()}),
        3 => Box::new(OddOrEvenGuppies{num_one : 0, num_two: 0}),
        4 => Box::new(Manyguppies{guppies_variants: many_var, current_variant: current_var}),
        _ => panic!("Please enter a valid number")
    };
    // Runs the game 
    run_game(currency, variant)
}
   
