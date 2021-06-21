// Primitive str = Inmutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure- Use when you need
// to modify our own string data.
pub fn run(){
    let mut hello = String::from("Hello");

    // Get length
    println!("Lenght: {}", hello.len());

    // Push char
    hello.push(' ');
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if variable is Empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}",word);
    }

    // Create string with Capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing.
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());

    println!("{}", s);

    println!("{}", hello);
}
