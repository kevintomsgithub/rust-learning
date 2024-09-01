use std::io::{self, Write};

struct Celsius(f64);
struct Fahrenheit(f64);

impl Celsius {
    pub fn to_fahrenheit(&self) -> Fahrenheit {
        let fahrenheit: f64 = (self.0 * 9.0 / 5.0) + 32.0;
        Fahrenheit(fahrenheit)
    }
}
impl Fahrenheit {
    pub fn to_celsius(&self) -> Celsius {
        let celsius: f64 = (self.0 - 32.0) * (5.0 / 9.0);
        Celsius(celsius)
    }
}

fn main() {
    println!(" ---- Welcome to C -> F converter ----");

    loop {
        println!("\n1. Celsius -> Fahrenheit");
        println!("2. Fahrenheit -> Celsius");
        println!("3. Exit");

        let mut opt = String::new();
        print!("Choose an option: ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut opt)
            .expect("Error reading value");

        match opt.trim() {
            "1" => {
                let c = Celsius(read_temparature());
                let f = c.to_fahrenheit();
                println!("Fahrenheit: {:.2}°F", f.0);
            }
            "2" => {
                let f = Fahrenheit(read_temparature());
                let c = f.to_celsius();
                println!("Celsius: {:.2}°C", c.0);
            }
            "3" => break,
            _ => println!("Invalid option. Please choose 1 or 2."),
        };
    }
}

fn read_temparature() -> f64 {
    let mut temparature = String::new();
    print!("temparature: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut temparature)
        .expect("Error reading temparature");
    match temparature.trim().parse() {
        Ok(num) => return num,
        Err(e) => panic!("Error parsing temparature - {e}"),
    };
}
