pub fn run() {
    println!("------------- Conditional.rs -------------");

    let age = 18;

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && age >= 18 {
        println!("Bartender: Sorry, you need to be 21 to drink in the US.");
    } else {
        println!("Bartender: Sorry, you're too young to drink.");
    }

    // Shorthand if
    let is_of_age = age >= 21;
    println!("Is of age: {}", is_of_age);
}
