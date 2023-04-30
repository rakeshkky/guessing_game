extern crate rand;
extern crate getopts;

use std::io;
use std::env;
use std::process;
use std::cmp::Ordering;
use rand::Rng;
use getopts::Options;

const LOWER_BOUND: u32 = 1;
const DEFAULT_UPPER_BOUND: u32 = 21;

fn gen_secret_number(upper_bound: u32) -> u32 {
    rand::thread_rng().gen_range(LOWER_BOUND, upper_bound)
}

fn validate_guess(guess: u32, upper_bound: u32) -> bool {
    guess >= LOWER_BOUND && guess < upper_bound
}

fn print_usage(program: &str, opts: Options) {
    let breif = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&breif));
}

fn parse_as_number(s: String) -> u32 {
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a number", s);
            process::exit(1)
        }
    }
}

fn main() {
    // TODO:- Use standard logging
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Options
    let mut opts = Options::new();
    opts.optopt(
        "u",
        "upper-bound",
        "set upper bound of guessing range",
        "NUMBER",
    );
    opts.optflag("h", "help", "print help menu");

    let opts_parsed = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };

    if opts_parsed.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let upper_bound = match opts_parsed.opt_str("u") {
        Some(s) => parse_as_number(s) + 1,
        None => DEFAULT_UPPER_BOUND,
    };

    // Actual program begin here
    println!("Guess the number!");

    let secret_number = gen_secret_number(upper_bound);
    let mut trials = 0;

    loop {
        // increment trials
        trials += 1;
        println!(
            "Please input your guess with a number between {} and {}",
            LOWER_BOUND,
            upper_bound - 1
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

        let is_valid = validate_guess(guess, upper_bound);

        if !is_valid {
            println!("Input number is not in range. Please try again!");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Equal! You win!");
                println!("No of trials: {}", trials);
                break;
            }
        }
    }
}
