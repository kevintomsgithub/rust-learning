use std::io::{self, Write};

fn main() {
    let x = read_int("Enter the first number (x): ");
    let y = read_int("Enter the first number (y): ");
    let op = read_operand("Enter the operand (+, -, *, /): ");

    let result = match op.as_str() {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0 {
                eprintln!("Error: Division by zero is not allowed.");
                return;
            }
            x / y
        }
        _ => {
            eprintln!("Error: Invalid operand '{op:?}'");
            return;
        }
    };

    println!("{x} {op} {y} = {result}");
}

fn read_int(prompt: &str) -> u32 {
    loop {
        print!("{prompt}");
        // To input the value on same line
        let _ = io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => eprintln!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn read_operand(prompt: &str) -> String {
    loop {
        print!("{prompt}");

        // To input the value on same line
        let _ = io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();
        if ["+", "-", "*", "/"].contains(&trimmed) {
            return trimmed.to_string();
        } else {
            eprintln!("Invalid operand. Please enter one of: +, -, *, /");
        }
    }
}
