use std::io;

fn main() {
    println!("Welcome to Fahrenheit <--> Celsius Temperature Converter!!");

    loop {
        let mut choice = String::new();
        println!("Enter 1 for Fahrenheit -> Celsius OR 2 for Celsius -> Fahrenheit: ");
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to get the choice from the input!");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid choice!!");
                return;
            },
        };

        let mut temperature = String::new();
        println!("Enter temperature value: ");
        io::stdin()
            .read_line(&mut temperature)
            .expect("failed to get temperature from the input!"); 
        

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                return;
            },
        };

        if choice == 1 {
            let temperature_in_fahrenheit = convert_to_fahrenheit(temperature);
            println!("{}",temperature_in_fahrenheit);
        } else {
            let temperature_in_celsius = convert_to_celsius(temperature);
            println!("{}",temperature_in_celsius);
        }
    }

}

fn convert_to_fahrenheit(temperature_in_celsius: i32) -> i32 {
    println!("Temperature in Fahrenheit -> ");
    return (temperature_in_celsius * 9 / 5) + 32;
}

fn convert_to_celsius(temperature_in_fahrenheit: i32) -> i32 {
    println!("Temperature in Cesius -> ");
    return (temperature_in_fahrenheit - 32) * 5 / 9;
}
