use std::env;

pub fn run () {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    //println!("Args: {}", command);

    let name = "Brad";
    let status = "100%";
    if command == "Hello" {
        println!("Hi {}, how are you?", name);
    }else if command == "status"{
        println!("Status is {}", status);
    }else {
        println!("This is not a valid command");
    }
}
