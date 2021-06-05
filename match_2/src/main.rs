fn main() {
    // match segunda parte

    for number in 1..31 {

        match (number % 3, number % 5){
            (0, 0) => println!("FIZZ BUZZ"),
            (0, _) => println!("FIZZ"),
            (_, 0) => println!("BUZZ"),
            _ => println!("{}", number)
        }
    }
        
}

