fn main() {

    // Slices -> Se desconoce su tamaño en tiempo de ejecución -> Heap
    // Arrays -> Se conoce su tamaño en tiempo de ejecución -> Stack

    let message = String::from("Hi world from Rust course!");
    println!("The message is: {}", message);

    let hi = &message[0..3]; // [index start..end]
    println!("The Slice of our String is: {}", hi);

    let rest_message = &message[3..message.len()];
    let rest_message_two = &message[3..];

    println!("The rest of our message is: {}", rest_message);
    println!("The rest of our message is: {}", rest_message_two);




}
