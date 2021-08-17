// const or static.

pub fn run(){
    const VALUE: i32 = 10;
    let mut number_one: i32 = 10;
    let number_two: i32 = 34;
    let result: i32 = number_one + number_two + VALUE;
    println!("The operation result is ({} + {} + {}) : {}", number_one, number_two, VALUE, result);
}
