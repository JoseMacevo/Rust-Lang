use std::io;
pub fn run(){

    let mut color = String::new();
    println!("Insert a color to traffic light: ");
    io::stdin().read_line(&mut color);

    let color = color.trim().to_lowercase();


    if color == "green"{
        println!("You can pass !!!");
    } else if color == "yellow"{
        println!("pass with caution !!!");
    } else if color == "red"{
        println!("STOP");
    } else{
        println!("Invalid color")
    }

}
