mod loops;
mod temperature_converter;
mod fibonacci;
mod ownership;
mod problems;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    // test loops
    loops::test_loops();
    
    // temp converter
    let mut _temp_f: f64 = 50.0;
    let mut _temp_c: f64 = 20.0;
    let temp_type: bool = false;
    temperature_converter::convert_temperature(&mut _temp_c, temp_type);
    println!("{}", _temp_c);
    
    // fibonacci number
    let fib_num = fibonacci::fibonacci(4);
    println!("fibonacci: {fib_num}");
    
    // ownership rules tests
    // ownership::test_double_free_error();
    ownership::test_clone();
    ownership::test_stack_copy();
    
    // guessing game
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The game isn't over yet...");
    let last_number: u32 = rand::thread_rng().gen_range(1..=10);
    println!("Go ahead...\nGuess the last number.");
    let mut user_guess = String::new();
    
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");
    loop {
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            },
        };
        
        match user_guess.cmp(&last_number) {
            Ordering::Less => println!("Too small.\nsuch failure."),
            Ordering::Greater => println!("Too big.\nfoolish overconfidence."),
            Ordering::Equal => println!("You win.\nThis time..."),
        };
        break;
    }
}