pub fn run () {
    // TUPLES.
    // Pueden almacenar diferentes tipos de datos al contrario que los arrays donde no es posible.

        //        0    1     2  -> Regla de los indices, comienzan en el 0.
    let tuple = (10, false, 5.5);  // -> Sin indicar el tipo de dato.
    println!("The tuple's value is: {:?} ", tuple);

    let mut tuple: (i32, bool, f64) = (10, false, 5.5);  // Indicando el tipo de dato.
    println!("The tuple's value is: {:?} ", tuple);

    // Obtencion de elementos de una tupla.

    let first_element = tuple.0;
    let last_element = tuple.2;

    tuple.1 = true;
    println!("The modificated tuple's value is: {:?} ", tuple);

    println!("The first element is: {:?}", first_element);
    println!("The last element is: {:?}", last_element);






}
