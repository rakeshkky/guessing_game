extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

const UPPER_BOUND: u32 = 101;
const LOWER_BOUND: u32 = 1;

fn gen_secret_number() -> u32 {
    rand::thread_rng().gen_range(LOWER_BOUND, UPPER_BOUND)
}

fn validate_guess(guess: u32) -> bool {
    guess >= LOWER_BOUND && guess < UPPER_BOUND
}

fn main() {
    println!("Guess the number!");

    //TODO:- take upper bound from command line args

    let secret_number = gen_secret_number();

    loop {
        println!(
            "Please input your guess with a number between {} and {}",
            LOWER_BOUND,
            UPPER_BOUND
        );

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line",
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's not a Integer. Please try again!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        let is_valid = validate_guess(guess);

        if !is_valid {
            println!("Input number is not in range. Please try again!");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Equal! You win!");
                break;
            }
        }
    }
}
