use std::mem;

pub fn run() {
    println!("------------- Vectors.rs -------------");
    let num = vec![1, 2, 3, 4, 5];

    println!("{:?}", num);
    println!("Single value: {}", num[0]);

    let mut num_2: Vec<i32> = vec![6, 7, 8, 9, 10];
    println!("{:?}", num_2);

    // Add to vector
    num_2.push(11);

    // pop off last value
    num_2.pop();

    // Remove index
    println!("Remove index 2: {}", num_2.remove(2));

    num_2[2] = 20;
    println!("{:?}", num_2);

    println!("{}", num_2[2]);

    println!("len: {}", num_2.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&num_2));

    // Get slice
    let slice: &[i32] = &num_2[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in num_2.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in num_2.iter_mut() {
        *x *= 2;
    }
    println!("Doubled: {:?}", num_2);
}
