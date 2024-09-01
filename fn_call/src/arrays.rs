use std::mem;

pub fn run() {
    let num = [1, 2, 3, 4, 5];

    println!("{:?}", num);
    println!("Single value: {}", num[0]);

    let mut num_2: [i32; 5] = [6, 7, 8, 9, 10];
    num_2[2] = 20;
    println!("{:?}", num_2);

    println!("{}", num_2[2]);

    println!("len: {}", num_2.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&num_2));

    // Get slice
    let slice: &[i32] = &num_2[0..2];
    println!("Slice: {:?}", slice);
}
