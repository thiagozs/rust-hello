pub fn run() {
    // Print to console
    println!("Hello from println.rs file!");

    // Basic formatting
    println!("Number: {}", 1);

    // Positional arguments
    println!("{} is from {}", "Thiago", "SC");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Thiago", "SC", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "Thiago", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}