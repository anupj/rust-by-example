use text_colorizer::*;
use std::io;
use std::env;
use std::fs;
// Let's write a function that parses a 
// buffer and returns the content of a csv 
// file into a Vec<Vec<String>>
fn parse_csv_document(src: impl io::BufRead) -> io::Result<Vec<Vec<String>>> {
    src.lines()
       .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully,
                // process it, if not, return the
                // error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // remove leading and trailing whitespace
                    .collect()
            })
       })
       .collect() // Collect all lines into a Vec<Vec<String>>
       // I don't understand where and how it returns the `Ok()` variant
}

#[derive(Debug)]
struct Arguments {
    filename: String,
}

// Parse the arguments provided by the user.
// We expect only one argument here - csv filename to read
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("{} wrong number of arguments: expected 1({}), got {}",
                   "Error".red().bold(), "file name".green().bold(), args.len());
        std::process::exit(1);
    } else {
        // if all is well with inputted args 
        // return Arguments
        Arguments {
            filename: args[0].clone(),
        }
    }
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    // Read data from the file at filename path
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                       "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let split_data = parse_csv_document(data.as_bytes()).unwrap();
    println!("{:?}", split_data);



}
