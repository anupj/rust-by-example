#![allow(unused)]

use std::num::{ParseFloatError, ParseIntError};

// with the return type rewritten, we use
// pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, 
             second_number_str: &str) ->
             Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

// As with `Option` we can use combinators such as `map()`
// This function is otherwise identical to the one above
fn multiply_v2(first_number_str: &str, 
             second_number_str: &str) ->
             Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| 
                                            first_number * second_number)
    })      
}

// v3 of multiply but with an early `return`
fn multiply_v3(first_number_str: &str, 
             second_number_str: &str) ->
             Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

// v4 of multiply using `?`
// `?` is almost exactly equivalent to an
// `unwrap` which `return`s instead of 
// `panic`king on `Err`s
fn multiply_v4(first_number_str: &str, 
             second_number_str: &str) ->
             Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = 
                multiply_v2("10", "2");
    print(twenty);

    let tt = 
                multiply_v2("t", "2");
    print(tt);
}
