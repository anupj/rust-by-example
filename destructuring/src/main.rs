fn main() {
    destructure_tuples();
    destructure_arrays_slices();
    destructure_enums();
    destructure_reference();
    destructure_structs();
    match_guard();
    variable_binding();
}

fn variable_binding() {
    let age: u32 = 18;
    println!("Tell me what type of person you are");

    match age {
        0 => println!("I haven't celebrated my 1st birthday yet."),
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    let some_number = Some(42);

    match some_number {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}

fn destructure_tuples() {
    let triple = (1, -2, 3);

    println!("Tell me more about {:?}", triple);

    // Match can be used to destructure a tuple
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}


fn destructure_arrays_slices() {
    let array = [8, -2, 4, 9, 6];

    match array {
        // Binds the second and third elements
        [0, second, third, ..] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // Single values can be ignored with _
        [1, _, third, ..] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),

        // Or store them in another array/slice 
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first
        // and the last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),


        
    }

}

#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name
    Red,
    Blue,
    Green,
    // these likewise tie `u32` tuples to different name: color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
fn destructure_enums() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {

        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

}

fn destructure_reference() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via des: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3; // plain old u8

    let ref _is_a_reference = 3;
    // if I try to println this, Rust will do
    // automatic dereferencing
    println!("the value for _is_a_reference is {}", _is_a_reference);

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value` : {:?}", m);
        },
    }
}

fn destructure_structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (3,2), y: 2 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}

enum Temperature {
    Celsius(i32),
    Farenheit(i32),
}

fn match_guard() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}c is above 30 celsius", t),
        Temperature::Celsius(t) => println!("{}c is below 30 celsius", t),
        
        Temperature::Farenheit(t) if t > 86 => println!("{}c is above 86 fahrenheit", t),
        Temperature::Farenheit(t) => println!("{}c is below 86 fahrenheit", t),


    }
}