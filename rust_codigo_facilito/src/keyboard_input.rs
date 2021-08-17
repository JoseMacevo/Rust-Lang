use std::io;

pub fn run(){
    println!("Please insert an username");
    let mut user_name = String::new();  // ->Static method.

    io::stdin().read_line(&mut user_name);// -> prestamo por referencia (&) y mutable (mut)

    let user_name = user_name.trim();

    println!("Please insert an user_name age:")

    let mut age = String::new();
    io::stdin().read_line(&mut age);
    let age = age.trim();

    let age: i32 = age.parse().unwrap();
    println!("The variable's value is {} and his age is {} years old.", user_name, age);
}
