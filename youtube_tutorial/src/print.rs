pub fn run(){
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Fromatting
    println!("{} is from {}", "Brad", "Mass");

    // Positonal arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Name arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 20, 20, 20);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);


}
