// Tuples group together values fo different types
// Max 12 elements.

pub fn run(){
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{} is from {} and has {} years old", person.0, person.1, person.2);
}
