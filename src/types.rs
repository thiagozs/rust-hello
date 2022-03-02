/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number)
Floats: f32, f64 (number)
Bool: true, false (boolean)
Characters: 'a' (char)
Tuples: (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128) (tuple)
Arrays: [i32; 3] (array)

Rust is a statically typed language, which means that the types of 
the variables must be known at compile time, and the compiler will 
not allow you to assign a value of the wrong type.

*/

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 50000;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
