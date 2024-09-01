pub fn run() {
    let person: (&str, &str, i8) = ("Kevin", "Massachusetts", 27);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let person_2 = ("John", "California", 30);
    println!("{:?}", person_2);
}
