fn main() {

    let message = Some("Hi World");

    match message {
        Some("Hi World") => println!("The message is Hi world."),
        Some("Good Bye") => println!("The message is Good Bye."),
        Some(_) => println!("This is another message"),
        None => println!("Inexistent value")

    }

}
