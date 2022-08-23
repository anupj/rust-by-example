fn create_fn() -> impl Fn() { // returns closure that implements trait Fn
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}


fn main() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding 
    // them to references
    // Annotation is identical to function 
    // annotation but is optional
    // as are the `{}` wrapping the body. 
    // These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    use std::mem;

    let color = String::from("green");

    // `print` gets an immutable borrow
    let print = || println!("`color: {}", color);

    print();

    // this re-borrow is possible because
    // the print closure only holds an
    // immut reference
    let _reborrow = &color;
    print(); // calling print again with no problem

    // this move is allowed because print is 
    // no longer used
    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // we can call the closure using a 
    // mutable borrow
    inc();
    inc();

    // The closure no longer needs to borrow `&mut count`. 
    // Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. 
    // A copy type would copy into the closure 
    // leaving the original untouched.
    // A non-copy must move and so `movable` immediately 
    // moves into the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so 
    // this can only be called once
    consume();

    // `Vec` has non-copy semantics
    let haystack = vec![1,2,3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&2));

    // declaring and calling functions
    // that return closures
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();


}
