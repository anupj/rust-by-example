fn main() {
   fizzbuzz(100); 
   iter();
   into_iter();
   iter_mut();
}

fn fizzbuzz(n: u32) {
    // `i` will take the values: 1, 2, ...., n (inclusive)
    // in each iteration
    for i in 1..=n {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean amongst us."),
            _ => println!("Hello {}", name),
        }
    }

    println!("unchanged `names` vector after using iter(): {:?}", names);
}

fn into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean amongst us."),
            _ => println!("Hello {}", name),
        }
    }

    println!("cannot use the `names` vector after using into_iter()");
}

fn iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean amongst us.",
            _ => "Hello",
        }
    }

    println!("Changed `names` vector after using iter_mut(): {:?}", names);
}

