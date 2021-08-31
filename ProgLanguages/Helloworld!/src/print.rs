pub fn run() {
    // Print to console 
    println!("Hello from print.rs file");
    

    // need placeholders for anything to print, basic formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "Ryan", "Pittsburgh");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Ryan", "Pittsburgh","code");

    // Named args
    println!("{name} likes to play {sport}", name = "Ryan", sport = "baseball");

    // Placeholder traits 
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}