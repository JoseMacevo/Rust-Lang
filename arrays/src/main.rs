fn main() {
    // Permiten almacenar múltiples valores en una coleccion
    // y deben almacenar el mismo tipo de dato.

    // Índices ->  0  1  2  3  4
    let numbers = [1, 2, 3, 4, 5];
    println!("The value of this array is: {:?}", numbers);

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // --> Especificamos el tipo de dato a contener
    // y su capacidad máxima.
    println!("The value of this array is: {:?}", numbers);

    let values = [5.5; 10]; // --> valores por defecto, tipo y capacidad máxima
    println!("The value of this array is: {:?}", values);

    let first_value = numbers[0];
    println!("The first element in this array is: {}", first_value);

    let last_value = numbers[numbers.len() -1];
    println!("The last element in this array is: {}", last_value);

    // Modificación de un elemento detro de un array.
    // El array ha de ser mutable

    numbers [2] = 30;
    println!("The array value is {:?}", numbers);

}
