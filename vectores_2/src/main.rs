fn main() {
    // Vectores segunda parte
    // Cuando se conozcan los valores que se van a guardar haremos uso de la macro.
    // Cuando se desconozcan los valores que se van a guardar haremos uso de la estructura.
    let mut vector: Vec<i32> = Vec::new(); // --> Creacón de un vector vacío con la estructura (Vec)
 
    vector.push(4);
    vector.push(5);
    vector.push(6);

    println!("El valor del vector es: {:?}", vector);
}
    
   
