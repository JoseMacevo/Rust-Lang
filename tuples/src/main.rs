fn main() {
    // Tuples
    // Nos permiten almacenar diferentes elementos dentro de una coleccion
    // Pueden almacenar diferentes tipos de datos, al contrario que los array
    
    //Índice ->  0    1     2
    let tuple = (1, true, 45.6); // -> Declaramos una tupla sin especificar su contenido
    println!("Our tuple´s values are: {:?}", tuple);

    let mut tuple: (i32, bool, f64) = (1, true, 45.6);// -> Declaramos una tupla indicando su contenido.
                                                  // y su longitud máxima (3 elementos)
    println!("Our tuple´s values are: {:?}", tuple);

    // Obtención de elementos de una tupla
    let last_element = tuple.2;
    let first_element = tuple.0;
    println!("The first element of our tuple is: {} and the last_element is: {}", first_element, last_element);
    
    // Modificación de los elementos de una tupla.
    tuple.1 = false;
    println!("Our new tuple is: {:?}", tuple);

    




}
 