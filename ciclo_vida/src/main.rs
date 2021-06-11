fn main() {
    let mut message = String::from("Hi I'm variable to loan");
    {

        let loan = &message;
        message = String::from("Value change");
        println!("{}", loan);
    }

    println!("{}", message)
}
