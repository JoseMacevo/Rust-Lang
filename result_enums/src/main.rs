enum ErrorDivision{
    Divisionbyzero,
    NegativesDivision,
}

fn division(number_one: i32, number_two: i32) -> Result<i32, ErrorDivision>{
    if number_two == 0 {
        return Err(ErrorDivision::Divisionbyzero);
    }

    if number_one < 0 || number_two < 0 {
        return Err(ErrorDivision::NegativesDivision);
    }

    Ok(number_one / number_two)
}

fn main() {

    let result = division(10, -3);

    match result {
        Ok(value) => println!("The result is : {}", value),

        Err(ErrorDivision::Divisionbyzero) => {
            println!("You can't divide by zero");
        }

        Err(ErrorDivision::NegativesDivision) => {
            panic!("You can't divide by a negative number.");

    }

    }
}
