use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter an expression: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let tokens: Vec<&str> = input.trim().split(' ').collect();
    let num1 = tokens[0].parse::<f64>().unwrap();
    let num2 = tokens[2].parse::<f64>().unwrap();
    let operator = tokens[1];
    match operator {
        "+" => println!("{} + {} = {}", num1, num2, num1 + num2),
        "-" => println!("{} - {} = {}", num1, num2, num1 - num2),
        "*" => println!("{} * {} = {}", num1, num2, num1 * num2),
        "/" => println!("{} / {} = {}", num1, num2, num1 / num2),
        _ => println!("Invalid operator"),
    }
}
