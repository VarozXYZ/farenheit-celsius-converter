use std::io;

fn main() {
    println!("Welcome to the temperature unit converter!");

    loop {
        println!("\nChoose conversion type:");
        println!("1. Celsius to Farenheit");
        println!("2. Farenheit to Celsius");
        println!("3. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let temp = get_temperature();
                let result = ctof(temp);
                println!("{temp} ºC is {result:.2} ºF");
            }

            "2" => {
                let temp = get_temperature();
                let result = ftoc(temp);
                println!("{temp} ºF is {result:.2} ºC");
            }

            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Please enter a valid choice"),
        }
    }
}

fn ftoc(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn ctof(temp: f32) -> f32 {
    (temp * 9.0 / 5.0) + 32.0
}

fn get_temperature() -> f32 {
    loop {
        println!("Please enter the temperature");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input");

        match temp.trim().parse::<f32>() {
            Ok(temp) => return temp,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
