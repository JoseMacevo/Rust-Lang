fn main() {
    // Constantes
    // const or static --> Palabra reservada para su creación.

    const VALUE: i32 = 10; //-> Obligatorio indicar el tipo de dato.
    let number_one: i32 = 10;
    let number_two: i32 = 20;
    let result = number_one + number_two + VALUE;
    println!("The result of this operation is: {}", result);
}
