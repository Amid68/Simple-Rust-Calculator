use std::io::{self, Write};

pub fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        }
    }
}

pub fn read_operator() -> char {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let op = input.trim().chars().next();
        match op {
            Some(c) if "+-*/".contains(c) => break c,
            _ => {
                println!("Please enter a valid operator (+, -. *. /).");
                continue;
            }
        }
    }
}


