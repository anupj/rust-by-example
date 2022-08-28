use std::fmt::Debug;

// A type `Borrowed` which houses a reference to an 
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this
// structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// --- Traits --- //
// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed2<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed2<'a> {
    fn default() -> Self {
        Self {
            x: &10, 
        }
    }
}

// -- Bounds -- //
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T`
// that has an unknown lifetime `'a`.
// `T` is bounded such that any *references* in `T`
// must outlive `a`. 
// Additionally, the lifetime of `Ref` may not 
// exceed `'a`.

// A generic function which prints using the `Debug` trait 
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in`T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
     T: Debug + 'a {
     println!("`print_ref`: t is {:?}", t);    
}

fn main() {
    let x = 18;
    let y = 15;
    // default integer type is i32

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    // -- Traits -- //
    let b: Borrowed2 = Default::default();
    println!("b is {:?}", b);

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
