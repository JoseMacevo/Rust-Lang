pub fn run(){
    // An array must hold the same data_type, only booleans, only integers, or only floats, etc, etc.
                // 0  1  2  3  4 -> Index Rules
    let numbers = [1, 2, 3, 4, 5]; // Size -> 5
    println!("The aray value is: {:?}", numbers);

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Indicate type and maximum capacity.
    println!("The aray value is: {:?}", numbers);

    let values = [5.5; 10];
    println!("The aray value is: {:?}", values);

    let first_element = numbers[0];
    println!("The first element is: {}", first_element);

    let last_element = numbers[numbers.len() -1];
    println!("The last element is: {}", last_element);

    numbers[2] = 30;
    println!("The aray value is: {:?}", numbers);





}
