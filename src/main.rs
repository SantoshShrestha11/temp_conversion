use std::io;

fn main() {
    println!("Temperature Converter");

    loop{
        println!("\nChoose an option:");
        println!("1: Convert celsius to Fahrenheit");
        println!("2: Convert Fahrenheit to Celsius");
        println!("3: Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid option");
                continue;
            }
        };

        match choice {
            1=> celsious_to_fahrenheit(),
            2=> fahrenheit_to_celsius(),
            3=> {
                println!("Goodbye!");
                break;
            }
            _=> println!("Invalid choice, please try again!");
        } 
    }
}

fn celsious_to_fahrenheit() {
    println!("Enter temperature in Celsius");

    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read input");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    let fahrenheit = (celsius*(9.0/5.0))+32.0;
    println!("{fahrenheit}F is equal to {celsius}C");
}



fn fahrenheit_to_celsius(){
    println!("Enter temperature in Fahrenheit");

    let mut fahrenheit= String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read input");
    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    let celsius = ((fahrenheit- 32.0) * 5.0)/9.0;
    println!("{celsius}C is equal to {fahrenheit}");
}

