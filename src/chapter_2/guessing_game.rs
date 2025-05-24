fn main() {
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
