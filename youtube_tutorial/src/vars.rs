// Variables hold primitive data or references to data
// Variables ara inmutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 37;
    println!("My mane is {} and I have {} years old", name, age);
    age = 38;
    println!("My mane is {} and I have {} years old", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple Variables

    let ( my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
    


}
