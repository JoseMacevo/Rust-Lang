/*
enum Option <T>{
    Some(T), -> El valor deseado
    None -> Ausencia de algún valor
}
*/
fn value_optaining(flag: bool) -> Option<String> {

    if flag {
        Some(String::from("I'm a message to some tuple!"))
    }

    else {
        None
    }

}

fn main() {
    //ENUMS
    // Option -> Si existe o no algún valor.
    // Result -> Errores -> panic!

    let result = value_optaining(false); // Option object.
//    match result {
//        Some(value) => println!("The value is: {}", value),
//        None => println!("Inexistent Value!!!")
//    }

// Obtención de lo que devuelve la tupla some mediante método unwrap y unwrap_or.

// let value = result.unwrap_or("This tuple is empty....!".to_string());
// println!("The value is: {}", value);

// Obtención del valor de la tupla "some" mediante el método expect.

let value = result.expect("A String was expected..!");
println!("The value is: {}", value);

}
