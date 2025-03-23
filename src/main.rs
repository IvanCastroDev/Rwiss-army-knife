mod math;
mod utils;

// Formulas:
// Celcius to farhenheit: F = (C * 9/5) + 32
// Farhenheit to celcius: C = (F - 32) * 5/9

const OPTIONS: [&str; 3] = ["Celcius to Fahrenheit", "Fahrenheit to Celcius", "Fibonacci Suquence"];

fn main () {
    println!("Rwiss army knife");

    loop {
        println!("Options:");
        for option in OPTIONS.iter().enumerate() {
            println!("{}.- {}", option.0 + 1, option.1);
        }

        let option = match utils::tools::get_input_and_parse("Input any option or press 'q' to exit: ") {
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
            1 => celcius_to_fahrenheit(),
            2 => farhenheit_to_celcius(),
            3 => fib_sequence(),
            _ => println!("{} is not a valid option", option),
        }
    }

    println!("Ending program");
}

fn fib_sequence() {
    println!("Fibonacci Suquence");

    loop {
        let n = match utils::tools::get_input_and_parse("Enter the number of elements in the sequence or input 'q' to get back to the main menu: ") {
            Ok(num) => num,
            Err(msg) => {
                if msg == "Exiting" {
                    break;
                }
                println!("{}", msg);
                continue;
            }
        };

        let result = math::operations::calc_fib_sequence(n);

        println!("The fibonacci sequence is: {:?}", result);
        continue;
    }
}

fn celcius_to_fahrenheit() {
    println!("Celcius to Fahrenheit");

    loop {
        let celcius = match utils::tools::get_input_and_parse("Enter the temperature in Celcius or input 'q' to get back to the main menu: ") {
            Ok(num) => num,
            Err(msg) => {
                if msg == "Exiting" {
                    break;
                }
                println!("{}", msg);
                continue;
            }
        };

        let result = math::operations::calc_celcius_to_fahrenheit(celcius);

        println!("The temperature in Fahrenheit is: {}", result);
        continue;
    }
}

fn farhenheit_to_celcius() {
    println!("Fahrenheit to Celcius");

    loop {
        let fahrenheit = match utils::tools::get_input_and_parse("Enter the temperature in Fahrenheit or input 'q' to get back to the main menu: ") {
            Ok(num) => num,
            Err(msg) => {
                if msg == "Exiting" {
                    break;
                }
                println!("{}", msg);
                continue;
            }
        };

        let result = math::operations::calc_fahrenheit_to_celcius(fahrenheit);

        println!("The temperature in Celcius is: {}", result);
        continue;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CELSIUS_CASES: &[(f32, f32)] = &[(0.0, 32.0), (100.0, 212.0), (37.0, 98.6)];
    const TEST_FAHRENHEIT_CASES: &[(f32, f32)] = &[(32.0, 0.0), (212.0, 100.0), (98.6, 37.0)];

    #[test]
    fn test_celsius_to_fahrenheit() {
        TEST_CELSIUS_CASES.iter().for_each(|&(c, f)| {
            assert_eq!(math::operations::calc_celcius_to_fahrenheit(c), f);
        });
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        TEST_FAHRENHEIT_CASES.iter().for_each(|&(f, c)| {
            assert_eq!(math::operations::calc_fahrenheit_to_celcius(f), c);
        });
    }

    #[test]
    fn test_fib_sequence() {
        let test_cases = [
            (0.0, vec![0]),
            (1.0, vec![0]),
            (2.0, vec![0, 1]),
            (3.0, vec![0, 1, 1]),
            (4.0, vec![0, 1, 1, 2]),
            (5.0, vec![0, 1, 1, 2, 3]),
        ];

        test_cases.iter().for_each(|&(n, ref expected)| {
            assert_eq!(math::operations::calc_fib_sequence(n), *expected);
        });
    }
}