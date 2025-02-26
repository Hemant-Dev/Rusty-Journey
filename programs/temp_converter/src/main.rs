use std::io;

fn main() {
    println!("Welcome to temp converter!!");

    let mut temperature = String::new();
    println!("Enter temperature value: ");
    io::stdin()
        .read_line(&mut temperature)
        .expect("failed to get temperature from the input!");
    
    let temperature: u32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        },
    };
    

    println!("Temperature is {temperature}");


}
