fn main() {
    // str -> Cadena de caracteres inmutable -> Stack
    // String -> Cadena de caracteres mutable -> Heap

    let variable_str = "Hi I'm str type";
    let variable_strig= String::new(); // -> String vacío
    let mut variable_strig2 = String::from("Hi I'm String type"); //String NO vacío.

    variable_strig2.push(',');
    variable_strig2.push(' ');
    variable_strig2.push('H');
    variable_strig2.push('E');
    variable_strig2.push('L');
    variable_strig2.push('L');
    variable_strig2.push('O');
    variable_strig2.push(' ');

    variable_strig2.push_str("We are in the Rust course of Código facilito");

    let new_string = "Hi, I'm a new String".to_string();

    let equal = new_string == "Hi, I'm a New String".to_string();


    println!("The str is: {}", variable_str);
    println!("The String is: {}", variable_strig);
    println!("The String is: {}", variable_strig2);
    println!("The New String is: {}", new_string);
    println!("Are this two strings equals?: {}", equal);



}
