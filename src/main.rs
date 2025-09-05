use std::io;
use std::mem;
use rand::Rng;

fn main() {
    //#[allow(dead_code)]
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
    
    struct Structure(i32);

    //println!("This struct `{:?}` won't print...", Structure(3));    

    let number: u32 = 30; 
    //let number2: u8 = 256; // i8 is not large enough to hold 256
    println!("number is: {number}");
    //println!("number2 is: {}", number2);
    println!("Max u8: {}, min u8: {}", std::u8::MAX, std::u8::MIN);
    //println!("but size of number2 is: {} bytes", mem::size_of_val(&number2));

}
