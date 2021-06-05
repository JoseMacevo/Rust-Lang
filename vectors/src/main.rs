fn main() {
    // Vectores
    // Permiten almacenar diferentes valores dentro de una colección
    // La mayor diferencia que existe con los arrays, es que los vectores
    // pueden modificar su longitud,
    // Y al igual que los arrays, sólo pueden almacenar un mismo tipo de dato
    // Índices -->    0  1  2
    let mut vector = vec![1, 2, 3]; // --> Macro vec! --> Hay que declararlo MUTABLE.
    
    // Añadir elementos a un vector
    vector.push(4); // Sin orden
    vector.push(5);
    vector.push(6);
    
    
    vector.insert(0, -1);  // Especificando órden y elemento a introducir.
    vector.insert(1, 0);
    
    // Eliminar elementos de un vector
    vector.remove(vector.len() -1);
    println!("El valor del vector es {:?}", vector);

    let first_element = vector[0];
    let last_element = vector [vector.len() -1];
    println!("The first element is: {} and the last element is: {}", first_element, last_element);
    
    vector[0]= -10; // -> Reasignación de un elemento por su índice
    println!("El valor del vector es {:?}", vector);

    let first_element = vector[0];
    let last_element = vector.pop().unwrap(); // -> Devuelve una estructura de tipo option (exito o error)
    println!("El valor del vector es {:?}", vector);
    println!("The first element is: {} and the last element is: {}", first_element, last_element);
    



    
    
}
