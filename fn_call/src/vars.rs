pub fn run() {
    let name = "Kevin";
    let mut age = 27;

    println!("My name is {} and I am {}", name, age);

    age = 28;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (name, age) = ("Kevin", 29);
    println!("My name is {} and I am {}", name, age);
}
