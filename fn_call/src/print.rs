pub fn run() {
    // print to console
    println!("Hello from print.rs");

    // Basic formatting
    println!("{} Number is {}", "The", 1);

    // Positional Arguments
    println!("{0} is a nice {1}. {0} is {2}", "Kevin", "guy", "awesome!");

    // Named Arguments
    println!(
        "How is {name} doing? How is {person} doing?",
        name = "Kevin",
        person = "John"
    );

    // Placeholder traits
    println!("Binary {:b} Hex {:x} Octal {:0}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (1, 2, "test", true));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
