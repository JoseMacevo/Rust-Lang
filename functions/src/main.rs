fn greet_users(){
    println!("Hi, from a function");
}

fn sum(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}
fn main() {

    greet_users();

    let result = sum(10, 20);
    println!("The result is {}", result);

}
