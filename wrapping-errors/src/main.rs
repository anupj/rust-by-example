use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    // Instead of defining it as a struct
    // `EmptyVec` is now a variant of 
    // `enum DoubleError`
    EmptyVec, 
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to 
// `DoubleError`. This will be automatically called by 
// `?` if a `ParseIntError` needs to be converted
// into a `DoubleError`.
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // Here we implicitly use the `ParseIntError` implementation of `From`
    // in order to create a `DoubleError`.
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!(" Caused by: {}", source);
            }
        },
    }
}


fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));

    // example with filter_map()
    let strings2 = vec!["tofu", "93", "18"];
    let numbers2: Vec<_> = strings2
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results for number 2: {:?}", numbers2);

    // example with map_err 
    let strings3 = vec!["tofu", "93", "18"];
    let mut errors = vec![];
    let numbers3: Vec<_> = strings3
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| 
            r.map_err(|e| errors.push(e)).ok()
        )
        .collect();
    println!("Results for number 3: {:?}", numbers3);

    // example with Result<Vec<_>, _>
    let strings4 = vec!["tofu", "93", "18"];
    let numbers4: std::result::Result<Vec<_>, _> = strings4
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results for number 4: {:?}", numbers4);

    // example with partition()
    let strings5 = vec!["tofu", "93", "18"];
    let (numbers5, errors5): (Vec<_>, Vec<_>) = strings5
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(std::result::Result::is_ok);
    println!("Results for number 5: {:?}", numbers5);
    println!("Errors for number 5: {:?}", errors5);
    // unwrap the `Result` to store the values in 
    // numbers5 and errors5 variables
    let numbers6: Vec<_> = numbers5.into_iter()
                                   .map(std::result::Result::unwrap)
                                   .collect();
    let errors6: Vec<_> = errors5.into_iter()
                                 .map(std::result::Result::unwrap_err)
                                 .collect();
    println!("Results for number 6 after unwrap: {:?}", numbers6);
    println!("Errors for number 6 after unwrap: {:?}", errors6);
}