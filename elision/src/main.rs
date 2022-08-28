// One input reference with lifetime
// `'a` which must live at least as
// long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with
// lifetimes as well
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: is {}, 
             y is {}", x, y);
}

// Returning references that have been passed 
// in is acceptable. However, the correct lifetimes
// must be returned
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a 
    // standalone function
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);
    
    let mut t = 3;
    add_one(&mut t); 

    print_one(&t);
}
