/*
Primitive types
Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128 (number of bits they take in memory)
Floats: f32, f64
Booleans(bool)
Characters (char)
Tuples
Arrays

Rust is a statically typed language, which means that it must know the types of all variables
at compile time, however, the compiler can usaually infer what type we want to use based on the
value and how we use it.
*/

pub fn run(){
    // Integer by default is "i32"
    let x = 1;

    // Float by default is "f64"
    let y = 2.5;

    // Add explicit types
    let z: i64 = 454545454554;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Booleans
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    // Character
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y , z, is_active, is_greater, a1, face));


}
