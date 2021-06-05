#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

impl User {
    fn greet(&mut self) {
        println!("Hi I'm {}", self.username);
    }

    fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}

fn main() {
    let mut user_one = User{

        username: String::from("user_one"),
        password: String::from("password"),

    };
    user_one.greet();
    user_one.change_password("New Password".to_string());

    println!("This user is: {:?}", user_one);
}
