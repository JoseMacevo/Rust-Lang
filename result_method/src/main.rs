/*
enum Result <T, E>{
Ok(T),
Err(E)
}
*/

#[derive(Debug)]

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

    // unwrap, unwrap_or, expect
    let result = division(10, 5);  // => Result
    let value = result.unwrap();
    println!("The result is: {}", value);

}
