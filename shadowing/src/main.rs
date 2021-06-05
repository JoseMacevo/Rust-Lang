fn main() {
    // Shadowing
    // Posibilidad de utilizar una misma variable la n cantidad de veces
    // necesarias.

    let value: i32 = 10;  // Inmutable.
    println!("The value of our variable is: {}", value);

    // A esta altura la variable (value) ya ha sido destruída.
    let value: i32 = 20;
    println!("The value of our variable is: {}", value);
    
    // A esta altura la variable (value) ya ha sido destruída.
    let value: bool = false;
    println!("The value of our variable is: {}", value);

}
