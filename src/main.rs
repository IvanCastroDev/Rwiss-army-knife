use std::{io};

// Formulas:
// Celcius to farhenheit: F = (C * 9/5) + 32
// Farhenheit to celcius: C = (F - 32) * 5/9

const OPTIONS: [&str; 3] = ["Celcius to Fahrenheit", "Fahrenheit to Celcius", "Fibonacci Suquence"];

fn main () {
    println!("Rwiss army knives");

    loop {
        println!("Options:");
        for option in OPTIONS.iter().enumerate() {
            println!("{}.- {}", option.0 + 1, option.1);
        }

        let option = match get_input_and_parse("Input any option or press 'q' to exit: ") {
            Ok(num) => num,
            Err(msg) => {
                if msg == "Exiting" {
                    break;
                }
                println!("{}", msg);
                continue;
            }
        }.round() as i32;

        match option {
            1 => celcius_to_fahrenheit_calc(),
            2 => farhenheit_to_celcius_calc(),
            3 => todo!("Work in progress, be patient"),
            _ => println!("{} is not a valid option", option),
        }
    }

    println!("Ending program");
}

fn celcius_to_fahrenheit_calc() {
    println!("Celcius to Fahrenheit");

    loop {
        let celcius = match get_input_and_parse("Enter the temperature in Celcius or input 'q' to get back to the main menu: ") {
            Ok(num) => num,
            Err(msg) => {
                if msg == "Exiting" {
                    break;
                }
                println!("{}", msg);
                continue;
            }
        };

        let result = calc_celcius_to_fahrenheit(celcius);

        println!("The temperature in Fahrenheit is: {}", result);
        continue;
    }
}

fn farhenheit_to_celcius_calc() {
    println!("Fahrenheit to Celcius");

    loop {
        let fahrenheit = match get_input_and_parse("Enter the temperature in Fahrenheit or input 'q' to get back to the main menu: ") {
            Ok(num) => num,
            Err(msg) => {
                if msg == "Exiting" {
                    break;
                }
                println!("{}", msg);
                continue;
            }
        };

        let result = calc_fahrenheit_to_celcius(fahrenheit);

        println!("The temperature in Celcius is: {}", result);
        continue;
    }
}

fn calc_fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn calc_celcius_to_fahrenheit(celcius: f32) -> f32 {
    (celcius * 9.0/5.0) + 32.0
}

fn get_input_and_parse(title: &str) -> Result<f32, String> {
    println!("{}", title);
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() == "q" {
        return Err("Exiting".to_string());
    }

    match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid number".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celcius_to_fahrenheit() {
        assert_eq!(calc_celcius_to_fahrenheit(0.0), 32.0);
        assert_eq!(calc_celcius_to_fahrenheit(100.0), 212.0);
        assert_eq!(calc_celcius_to_fahrenheit(37.0), 98.6);
    }

    #[test]
    fn test_fahrenheit_to_celcius() {
        assert_eq!(calc_fahrenheit_to_celcius(32.0), 0.0);
        assert_eq!(calc_fahrenheit_to_celcius(212.0), 100.0);
        assert_eq!(calc_fahrenheit_to_celcius(98.6), 37.0);
    }
}
