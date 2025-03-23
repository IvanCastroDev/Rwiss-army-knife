use std::io;

pub fn get_input_and_parse(title: &str) -> Result<f32, String> {
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