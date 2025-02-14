use std::io;

fn main(){
    println!("Guessing Game!");
    println!("Please enter the guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input.");
    
    println!("You guessed, {}", guess);
}