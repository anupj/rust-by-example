#![allow(unused)]
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn double_first_v2(vec: Vec<&str>) -> 
                Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("------------------------------");
    println!("double_first that returns `Option<Result<i32, ParseIntError>>`");
    println!("The first doubled with numbers is {:?}", double_first(numbers.clone()));

    println!("The first doubled with empty is {:?}", double_first(empty.clone()));

    println!("The first doubled with strings is {:?}", double_first(strings.clone()));

    println!("------------------------------");

    println!("double_first that returns `Result<Option<i32>, ParseIntError>`");
    println!("The first doubled with numbers is {:?}", double_first_v2(numbers));

    println!("The first doubled with empty is {:?}", double_first_v2(empty));

    println!("The first doubled with strings is {:?}", double_first_v2(strings));

}
