use std::io;

fn main() {

    let mut color = String::new();
    println!("Insert a color to the traffic light: ");

    io::stdin().read_line(&mut color);  //Préstamo por referencia con mutabilidad
    let color = color.trim().to_lowercase(); // Eliminamos el salto de línea y estandarizamos la entrada a minúscula.
    
    if color == "green" {

        println!("You can pass!!!")
    
    }

    else if color == "yellow" {

        println!("Be carefull....")
    }

    else if color == "red" {
        println!("STOP you can´t pass")
    }

    else {
        println!("Color isn´t defined")
    }
}

