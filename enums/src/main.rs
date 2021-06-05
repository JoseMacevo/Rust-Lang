fn main() {
    // Enum --> CamelCase
    // Tipo el cual nos permite almacenar diferentes opciones.

    enum Response {
        Success, 
        Error(u32, String), // 403, 404, 500
    }

    let answer = Response::Error(501, String::from("Isn´t possible complete this operation"));

    match answer {
        Response::Success => println!("The request was made correctly."),
        Response::Error(403, _) => println!("Forbidden"), 
        Response::Error(404, _) => println!("Not Found"), 
        Response::Error(500, _) => println!("Internal server error"),
        Response::Error(_, message) => println!("{}",message)
        
    
    }
}
