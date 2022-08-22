enum Foo {
    Bar,
    Baz,
    Qux(u32)
}
fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 50) = c {
        println!("this is not going to bind for {:?}", value);
    }

    // lets also try out `while let` in here
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is {:?}. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
