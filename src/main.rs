use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::{thread, time};

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!!"),
            Ordering::Greater => println!("Too Large!!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
    println!("This program will automatically close in 2 seconds...");
    let two_secs = time::Duration::new(2, 0);
    thread::sleep(two_secs);
}
