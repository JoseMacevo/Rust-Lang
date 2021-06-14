#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    age: Option<u32>
}


fn main() {
    //Implementación del enum Option en las estructuras.
    let user_one = User {
        username: String::from("Eduardo"),
        password: String::from("password"),
        email: String::from("Eduardo.codigofacilito.com"),
        // age: Some(26)
        age: None

    };

    println!("The user is: {:?}", user_one);

    // let age = user_one.age.unwrap(); // Porque conocemos la edad, en caso de no conocerlo se utilizaría match.
    match user_one.age{
        Some(age) => println!("His age is: {} years old.",age),
        None => {},
    }

    // println!("His age is: {} years old.", age);

}
