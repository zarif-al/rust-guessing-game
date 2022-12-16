use std::{ io, cmp::Ordering };
use rand::Rng;

fn main() {
    // The =100 makes it so that the range includes 100.
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to the guessing game");

    loop {
        println!("Guess the number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
            Ok(num) => { num }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}