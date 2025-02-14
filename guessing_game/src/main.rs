use std::io;
use rand::Rng;

fn main(){
    println!("Guessing Game!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is {secret_number}");
    
    println!("Please enter the guess!");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input.");
    
    println!("You guessed, {}", guess);
}