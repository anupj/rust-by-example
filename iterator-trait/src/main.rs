struct Fibonacci {
    curr: usize,
    next: usize,
}

// Implement `Iterator` for `Fibonacci`
// The `Iterator` trait only requires a method to be defined
// for the `next` element
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = usize;

    // Here, we define the sequencee using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there is no endpoint to a Fib sequence, 
        // the `Iterator` will never return `None`, and 
        // `Some` is always returned
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0usize, next: 1usize }
}

fn main() {
     // `0..3` is an `Iterator` 
     // that generates: 0, 1, 2
     let mut sequence = 0..3;
     
     println!("Four consecutive `next` calls on 0..3");
     println!("> {:?}", sequence.next());
     println!("> {:?}", sequence.next());
     println!("> {:?}", sequence.next());
     println!("> {:?}", sequence.next());

     // `for` works through and `Iterator` 
     // until it returns `None`
     // Each `Some` value is unwrapped 
     // and bound to a variable (here, `i`)
     for i in 0..3 {
        println!("> {}", i);
     } // important to note 
     // that `for` does the unwrapping for us

     // The `take(n)` method reduces an `Iterator` 
     // to its first `n` terms
     println!("The first four terms of the 
                Fibonacci sequence are: ");
    for i in fibonacci().take(14) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator`
    // over an array/slice
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
