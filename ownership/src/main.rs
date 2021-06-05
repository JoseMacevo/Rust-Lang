struct Rectangle {
    width: u32,
    height: u32

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // Ownership -> Permite garantizar la integridad de la memoria sin un garbagecollector.

     /*
     * Cada valor en Rust tiene su propio Ownership.
     * Sólo puede existir un Ownership a la vez.
     * Si un Ownership sale del alcance, el valor se descartará.
     */

    let rectangle = Rectangle {width: 10, height: 20};
    // Los argumentos son pasados mediante préstamos -> Cedemos el Ownership de las variables.
    // Los argumentos pueden ser pasados mediante referencias (&) -> Se devuelve el Ownership.
    let result = area(&rectangle);
    println!("The Rectangle's area is: {}", result);
    println!("The Rectangle's width and height are: {} - {}", rectangle.width, rectangle.height);


}
