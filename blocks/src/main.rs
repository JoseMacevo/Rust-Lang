fn main() {
    // Bloques --> Colección de sentencias agrupadas dentro de un juego de llaves.
    // Una variable, se crea, utiliza, modifica y destruye siempre dentro de un bloque 

    let message = "Hi...I´m a variable inside the main block";
    println!("{}", message);

    {
        println!("Hi.... from a second block....");
        println!("{}--> But in son block", message);
    }
    
}
 // Finalizado el bloque las variables son destruídas

 //Seguir con el video en el minuto 04:00
