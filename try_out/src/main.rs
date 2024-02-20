fn main() {
    let age: i32 = 12;
    match age {
        1..=10 => println!("Age between 1 and 10"),
        21 | 32 => println!("Your age is 21 or 32"),
        50..=i32::MAX => println!("Age between 50 and {}", i32::MAX),
        _ => print!("You don't have birthday!")
    };
}
