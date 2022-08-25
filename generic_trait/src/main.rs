// Non-copyable types
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter
// `T` and caller `U`
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed
    // arguments, deallocating both
    fn double_drop(self, _: T) {
    }
}

use core::fmt::Display;
// Example of a trait bound
// Generic type `T` is bound by `Display`
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

trait PrintInOption {
    fn print_in_option(self);
}

use std::fmt::Debug;
impl<T> PrintInOption for T where
    Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }


fn main() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`
    empty.double_drop(null);
    
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey  = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
