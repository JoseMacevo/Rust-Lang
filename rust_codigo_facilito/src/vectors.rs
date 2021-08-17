pub fn run(){
    //VECTORS.
    // Permiten almacenar un mismo tipo de dato dentro de una coleccion, a diferencia de los arrays
    // Los vectores permiten modificar su longitud.


                //        0  1  2 -> Regla de los indices, empiezan con el valor 0.
    let mut vector = vec![1, 2, 3];  // -> Agrega elementos al final del vector.
    println!("The vector's value is: {:?}", vector);

    vector.push(4);
    vector.push(5);
    println!("The vector's value is: {:?}", vector);

    vector.insert(0, -1);
    vector.insert(1, 0);
    println!("The vector's value is: {:?}", vector);

    vector.remove(vector.len()-1);
    println!("The vector's value is: {:?}", vector);

    let first_element = vector[0];
    //let last_element = vector.len()-1;
    let last_element = vector.pop().unwrap();

    println!("The first element of our vector is: {}", first_element);
    println!("The last element of our vector is: {}", last_element);

    vector[0] = -10;
    println!("Our modificated vector is: {:?}", vector);


}
