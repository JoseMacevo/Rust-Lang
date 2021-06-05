struct User{
    username: String,
    password: String
}

fn create_user(username: String, password: String) -> User {
    User {username, password}

}

fn main() {
    let username = String::from("Jose");
    let password = String::from("password123");
    let mut user = create_user(username, password);
    user.username = String::from("Value change");
    println!("The username is: {}", user.username);
    println!("The password is: {}", user.password);

}
