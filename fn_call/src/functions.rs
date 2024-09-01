pub fn run() {
    println!("------------- Functions.rs -------------");

    greeting("Hello", "Kevin");
    let s = add(5, 5);
    println!("Sum: {}", s);

    // Closure - similar to lambda functions in Python
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// Functions with return values
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // No semicolon means it's an expression and will return the value
}
