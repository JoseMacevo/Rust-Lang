fn main() {

    // Ciclo while --> Ciclo indeterminado.
    // Debemos utilizarlo cuando desconozcamos el número de iteraciones que haremos.
    let mut number = 123456789;
    let mut counter = 0;

    while number > 0 {
        number = number / 10;
        counter += 1;
    }
    println!("The quantity of numbers are: {}", counter);
}
