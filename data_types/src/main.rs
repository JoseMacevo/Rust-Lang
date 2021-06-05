fn main() {
    // Data_Types
    //Str or String requieren una clase a parte.
    // Rust es un lenguaje de programación fuertemente tipado.
    // i8, i16, i32, i64, i128 -> Con signo + o -
    // u8, u16, u32, u64, u128 -> Sin signo o +
    let number_one: i8 = -10;
    let number_two: u8 = 10;
    println!("{} {}", number_one, number_two);
    
    // Char -> Caracter -> utf_8
    let caracter = 'a';
    println!("El caracter es: {}", caracter);

    // Floats -> f32 or f64
    let real: f32 = 12.5;
    println!("El valor del flotante es: {}", real);

    // Booleans -> true or false
    let result: bool = false;
    println!("El valor del booleano es: {}", result);
    
}
