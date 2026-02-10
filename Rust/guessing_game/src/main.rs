use rand::prelude::*;
use std::io;
use std::{thread::sleep, time::Duration};

fn main() {
    println!("Hello Guesser!");
    sleep(Duration::from_secs(2));
    println!("Guess the number between 1 to 10!");

    let rng = rand::rng().random_range(1..=10);
    let mut guess = String::new();

    println!("{rng}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Pls type an actual number pls!");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    if guess == rng {
        println!("You are right!");
    } else {
        println!("You are wrong! Now die!");
    }
}
