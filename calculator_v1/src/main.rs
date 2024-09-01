use std::io::{self, Write};

fn main() {
    println!("Enter 2 numbers");

    // Input value of x
    print!("x: ");
    let mut x = String::new();
    // To input the value on same line
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut x)
        .expect("Error reading value of 'x' from STDIN");
    let x: i32 = match x.trim().parse() {
        Ok(x) => x,
        Err(e) => panic!("Error parsing 'x', {e:?}"),
    };

    // Input value of y
    print!("y: ");
    let mut y = String::new();
    // To input the value on same line
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut y)
        .expect("Error reading value of 'y' from STDIN");
    let y: i32 = match y.trim().parse() {
        Ok(y) => y,
        Err(e) => panic!("Error parsing 'y', {e:?}"),
    };

    // Input operand
    print!("operand: ");
    let mut op = String::new();
    // To input the value on same line
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut op)
        .expect("Error reading operand from STDIN");
    let op: &str = op.trim();

    let result = match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Operation cannot be performed."),
    };

    println!("{x} {op} {y} = {result}");
}
