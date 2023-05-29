use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..3);
    println!("GUESS THE NUMBER");

    for _n in 1..3 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Something went wrong, please try again");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            // println!("{} {} !", "it".green(), "works".blue().bold());
            Ordering::Less => println!("{}!", "Too small".yellow().bold()),
            Ordering::Greater => println!("{}!", "Too big".red().bold()),
            Ordering::Equal => {
                println!("{}!", "You win".green().bold());
                break;
            }
        }
    }
    println!("Correct number {}", secret_number);
}
