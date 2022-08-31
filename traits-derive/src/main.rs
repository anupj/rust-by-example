#![allow(unused_parens)]
// `Centimeters`, a tuple struct that can be
// compared
#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be
// printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        // this is how you get access to the
        // value inside the tuple
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional 
// attributes
#[derive(PartialEq)]
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);
    let _this_is_true = 
                 ( _one_second == _one_second);
    
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    println!("One foot in centimeters is {:?}", 
               foot.to_centimeters());

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    
    println!("One foot is {} than one meter", cmp);

}
