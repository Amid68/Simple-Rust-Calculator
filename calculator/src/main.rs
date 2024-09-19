mod operations;
mod utils;

use utils::input;

fn main() {
    println!("Simple Rust Calculator");
    println!("Enter the first number:");
    let num1 = input::read_number();
    println!("Enter an operator (+, -, *, /):");
    let operator = input::read_operator();
    println!("Enter the second number:");
    let num2 = input::read_number();

    let result = match operator {
        '+' => operations::add(num1, num2),
        '-' => operations::subtract(num1, num2),
        '*' => operations::multiply(num1, num2),
        '/' => operations::divide(num1, num2),
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}
