fn main() {
    
    let number: i32 = 5;
    match number {
        // valor => sentencia a ejecutar o tarea a realizar.
        1 => println!("The number is one."),
        2 => println!("The number is two."),
        3 => println!("The number is three"),
        4 | 5 | 6 => println!("The number is in between four and six"),
        _ => println!("{}", number) // Default
    }
}
