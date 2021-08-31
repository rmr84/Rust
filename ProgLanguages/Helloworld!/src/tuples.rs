// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Ryan", "Pittsburgh", 21);

    println!("{} is from {} and is {}", person.0, person.1, person.2);


}