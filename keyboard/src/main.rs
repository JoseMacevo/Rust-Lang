use std::io;  // --> Librería estandard de Rust.

fn main() {
    // Introducción por teclado.
    // del nombre.
    
    println!("Insert user name:");
    let mut user_name = String::new();  // (Static method) --> No necesita ser instanciado
    // --> para ser ejecutado, devuelve un String vacío.
    io::stdin().read_line(&mut user_name); // Préstamo de user_name por referencia con permisos de mutabilidad.
    // read_line devuelve un valor de tipo (Result) --> El cual puede poseer dos estados exito o error.
    let user_name = user_name.trim();
    
    // de la edad.
    
    println!("Insert user age: ");
    let mut user_age = String::new();
    io::stdin().read_line(&mut user_age);
    let user_age = user_age.trim();
    let user_age: i32 = user_age.parse().unwrap(); // De String a entero
    // parse devuelve un valor de tipo (Result) --> El cual puede poseer dos estados exito o error.
    println!("La edad de {} son {} años", user_name, user_age);
    
}



