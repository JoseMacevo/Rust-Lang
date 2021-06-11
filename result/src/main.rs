/*
enum Result<T, E>{
    Ok(T),
    Err(E)
}
*/
fn division(number_one: i32, number_two: i32) -> Result<i32, String>{
    if number_two == 0{
        return Err(String::from("Zero Division Error"));
    }
    Ok(number_one / number_two)
}


fn main() {
    // Result -> Trabajar con errores.
    let result = division(10, 0);

    match result {
        Ok(value) => println!("The result is: {}", value),
        Err(error) => println!("The error is: {}", error)

    }

}
