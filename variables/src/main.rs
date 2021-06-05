fn main() {
    
    //Estos son comentarios simples
    // Variables son inmutables por defecto, si se necesita modificar su valor con 
    // posterioridad a su declaración será necesario usar la palabra (mut) a la hora
    // de crearla.
    // let <nombre de la variable> = <valor>

    // Estos son comentarios de líneas múltiples.
    /* por ejemplo este sería
    uno de ellos*/
    
    let number_one = 10;  // Declaracion de una variable sin indicar su tipo.
    let mut number_two: i32 = 10;  //Declaración de una variable indicando su tipo (entero de 32bits).
    number_two = 100;
    let result = number_one + number_two;
    println!("The result of our operation is {} plus {} : {}", number_one, number_two, result);

}



 
