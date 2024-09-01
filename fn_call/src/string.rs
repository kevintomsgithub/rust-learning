pub fn run() {
    let hello = "Hello";
    println!("Primitive string: {}, len: {}", hello, hello.len());

    let mut hello_mut = String::from("Hello ");

    hello_mut.push('W'); // push a character

    hello_mut.push_str("orld!"); // push a string

    println!("{}", hello_mut);

    // Capacity in bytes
    println!("Capacity: {}", hello_mut.capacity());

    // Check if empty
    println!("Is empty: {}", hello_mut.is_empty());

    // Contains
    println!("Contains 'World' {}", hello_mut.contains("World"));

    // Replace
    println!("Replace: {}", hello_mut.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_mut.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
}
