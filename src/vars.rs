// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Thiago";
    let mut age = 1;
    println!("My name is {} and I am {}", name, age);
    age = 2;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Thiago", 1);
    println!("{} is {}", my_name, my_age);
    
}