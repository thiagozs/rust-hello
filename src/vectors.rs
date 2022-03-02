use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Re-assign values
    numbers[2] = 20;

    // Add to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get first value
    println!("First value: {}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2; // multiply by 2
    }

    println!("Numbers Vec: {:?}", numbers);

}