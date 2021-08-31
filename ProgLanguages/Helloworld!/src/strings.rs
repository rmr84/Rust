// Primitive str = immutable fixed-length string somewhere in mem
// String = growable, heap-allocated data structure - use when needed to modify
// or own string data

pub fn run() {
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    // Get length 
    println!("Length: {}", hello.len());
    
    // push a char
    hello.push('W');
    println!("{}", hello);

    // push a string
    hello.push_str("orld!");
    println!("{}", hello);

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));
    
    // Loop through string by whitespace 
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
}