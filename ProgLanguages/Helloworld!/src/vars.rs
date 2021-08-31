// cariables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "Ryan";
    let mut age = 21;

    println!("My name is {} and I am {}", name, age);
    age = 22;
    println!("My name is {} and I will be {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age, ) = ( "Ryan", 21);
    println!("{}, {}", my_name, my_age);

}