use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn user_picks_number() {
    println!("Pick a number between 1 and 100 and I'll try to guess it!");

    let mut low = 1;
    let mut high = 100;

    loop {
        let guess = (high + low) / 2;

        println!("Is {guess} your number? (y/n):");

        {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.contains("y") {
                println!("Hooray! Thanks for playing.");
                break;
            }
        }

        println!("Was {guess} too high or too low? (low/high):");

        {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.starts_with("low") {
                low = guess + 1;
            } else {
                high = guess - 1;
            }
        }
    }
}

fn computer_picks_number() {
    println!("I'm thinking of a number between 1 and 100...");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Try again.");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Hooray! You got it.");
                break;
            }
        }
    }
}

fn main() {
    println!("Hello, and welcome to the guessing game.");
    println!("Would you like to guess a number I pick (1) or should I guess your number (2)?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    if input.contains("1") {
        computer_picks_number();
    } else {
        user_picks_number();
    }
}
