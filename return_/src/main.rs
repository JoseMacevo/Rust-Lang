// Reserved word RETURN

fn suma(number_one: i32, number_two: i32) -> i32 {
        number_one + number_two
    }
    fn factorial(number: u32) -> u32 {
        if number == 1 {
            return number;
        }

        factorial(number - 1) * number

    }

    fn main(){

        let result = factorial(5);
        println!("The factorial is: {}", result)


    }
