// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Thiago", "SC", 1);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {}", y);

    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];

    // println!("The value of first is: {}", first);
    // println!("The value of second is: {}", second);
}