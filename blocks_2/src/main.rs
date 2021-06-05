fn main() {
    // Los bloques pueden devolver valores.
    
    let calification: i8 = 10;

    let message = if calification == 10 {
        String::from("Congrats...")
    }
     else {
        String::from("You need more hours of study....")
     };
    
     println!("{}", message);
}
