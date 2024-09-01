use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Secret is {secret_number}.");

    loop {
        println!("Enter a number 1-10: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error parsing number {e}. Try again!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too high."),
            Ordering::Less => println!("Too low."),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        };
    }
}
