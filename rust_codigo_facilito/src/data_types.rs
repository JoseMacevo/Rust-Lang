pub fn run(){
    /*numerics
        // i8, i16, i32, i64, i128 -> Negatives and positive numbers.
        // u8, u16, u32, u64, u128 -> Only positive numbers.
    */

   let number_one: i8 = -10;
   let number_two: u8 = 10;
   println!("{} {}", number_one, number_two);

   // char -> UTF-8

   let caracter = 'a';
   println!("{} {} {}", number_one, number_two, caracter);

   // Floats. f32 o f64

   let real_value: f32 = 12.5;
   println!("{} {} {} {}", number_one, number_two, caracter, real_value);

   //Booleans -> true or false

   let result: bool = false;
   println!("{} {} {} {} {}", number_one, number_two, caracter, real_value, result);



}
