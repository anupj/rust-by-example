macro_rules! create_function {
    // This macro takes an argument of designator `ident` 
    // and creates a function named `$func_name`.
    // The `ident` designator is used for 
    // variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify` macro converts 
            // and `ident` into a string
            println!("You called {:?}",
                    stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr`
    // and prints it as a string along with 
    // its result. The `expr` designator 
    // is used for expressions.
    ($expression:expr) => {
        // `stringify` will convert the
        // expression *as it is* into a
        // string
        println!("{:?} = {:?}", 
                stringify!($expression),
                $expression);
    };
}

// Overload
// `test!` will compare `$left` and `$right`
// in different ways depending on how to invoke it
// This macro has 2 versions - `and` version and 
// `or` version
macro_rules! test {
    // Arguments don't need to be
    // separated by a comma. Any template
    // can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right) // what does && do again?
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}


// Repeat
// `find_min!` will calculate the minimum
// of any number of arguments
macro_rules! find_min {
    // Base case
    ($x:expr) => ($x);

    // `$x` followed by at least
    // one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    ) 
}

fn main() {
    // Designators testing
    foo();
    bar();

    print_result!(1u32 +1);
    // blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x +2 * x - 1

    });

    // Overload
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    // Repeat
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
