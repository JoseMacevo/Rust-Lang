
// The variables in rust are inmutables by default.
// If we want create a mutable variable we have to use the word (mut), when we create the variable.
// let <variable's name> = <the value>  -> Declaracion de una variable.
// let <variable's name> : <Value type> = <the value> -> Declaracion de una variable idicando el tipo de dato.
// Comentario de una sola linea.
/*Comentario de
varias lineas de codigo
*/



pub fn run(){

    let mut number_one = 10; // -> Ejemplo de declaracion de una variable sin indicar el tipo de dato a almacenar.
    let number_two: i32 = 10; //-> Ejemplo de declaracion de una variable indicando el tipo de dato a almacenar.
    number_one = 100;
    let result = number_one + number_two;
    println!("The result of adding ({} and {}) is: {}", number_one, number_two, result);

}
