use std::io::{self, Write};

fn main() {
    println!(" ---- Welcome to C -> F converter ----");

    let mut celsius = String::new();
    print!("Celsius: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Error reading value");
    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing celsius - {e}"),
    };

    println!("Celsius: {celsius:.2}°C");

    let fahrenheit = convert_c_to_f(celsius);
    println!("Fahrenheit: {fahrenheit:.2}°F");
}

fn convert_c_to_f(celsius: f64) -> f64 {
    let fahrenheit: f64 = (celsius as f64 * 9.0 / 5.0) + 32.0;
    fahrenheit
}
