#[warn(dead_code)]
fn main() {
    general_println();
    general_display();
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn general_display() {
    println!("inside general_display()");
    println!("");

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // println
    println!("{:#?}", peter);

}


fn general_println() {
    println!("inside general_println()");
    println!("");
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can invoked by specified format character after a
    // `:`.
    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);


    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types to not implement fmt::Display by default

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    println!("This struct `{:?}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    // For Rust 1.58 and above, you can directly capture the argument from
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");

    let name: &str = "anup jadhav";
    println!("Hiya! My name is {name} and my height is {width} inches");


    // Lets print out the value of PI
    const PI: f64 = 3.14159265359;
    println!("PI is roughly {PI:.4}");


}

use std::fmt;

// This is a tuple struct named `Structure` that contains an i32
struct Structure(i32);

// To use `{}` marker the trait `fmt::Display` must be implemented
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}