// command line information
use std::env;

pub fn run() {
    //gets target of executable 
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Ryan";
    let status = "100%";

    //println!("Command: {:?}", command);
    
    if command == "main" {
        println!("Hi {}. how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);

    }

}