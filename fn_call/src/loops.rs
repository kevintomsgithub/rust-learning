pub fn run() {
    println!("------------- Loops.rs -------------");

    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 10 {
            break;
        }
    }

    // While loop (FizzBuzz)
    count = 0;

    while count <= 5 {
        count += 1;
        println!("Count: {}", count);
    }

    // For range
    for x in 0..5 {
        println!("Range: {}", x);
    }
}
