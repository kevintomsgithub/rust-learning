// Similar to objects in JS, classes in Java, Python, etc.

// Create a new user
struct User {
    email: String,
    username: String,
    active: bool,
}

// Tuple struct
struct Color(i32, i32, i32);

pub fn run() {
    println!("------------- Structs.rs -------------");

    let mut user1 = User {
        email: String::from("Toms"),
        username: String::from("kevin"),
        active: true,
    };

    // Print user1
    println!("{}, {}, {}", user1.email, user1.username, user1.active);

    // Print user1 email
    println!("{}", user1.email);

    // Change user1 email
    user1.email = String::from("Test");
    println!("{}, {}, {}", user1.email, user1.username, user1.active);

    // Tuple struct
    let black = Color(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);
}
