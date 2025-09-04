use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess : ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    if guess == secret_number.to_string() {
        println!("You win!");
    } else {
        println!("You lose! The secret number was: {}", secret_number);
    }
    
}
