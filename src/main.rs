use rand::Rng;
use std::io;

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("your secret number is: {}", secret_number);
    println!("input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
