use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut total_guesses: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Type Result = Either
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Match
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                total_guesses += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                total_guesses += 1;
            }
            Ordering::Equal => {
                println!("You win!");
                total_guesses += 1;
                break;
            }
        }
    }
    println!("The secret number is {secret_number}.");
    println!("You took {total_guesses} guesses to find it.");
}
