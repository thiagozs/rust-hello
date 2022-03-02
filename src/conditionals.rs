// Conditional: if-else

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("You can drink");
    } else if age < 21 && check_id {
        println!("You cannot drink");
    } else {
        println!("Ill need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);

}