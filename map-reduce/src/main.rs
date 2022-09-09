use std::thread;

// This is the `main` thread
fn main() {
    // This is our data to process
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn
    let mut children = vec![];

    /************************************************************
     * "Map" process
     *
     * Divide our data into segments, apply initial processing
     * **********************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual 
    // data 
    let chunked_data = data.split_whitespace();

    // Iterate over the data segements.
    // .enumerate() adds the current loop index to 
    // whatever is iterated.
    // The resulting tuple "(index, element)" is then 
    // immediately "destructured" into two variables, "i" and
    // "data_segment" with a "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate threads
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // `move || -> u32` is syntax for a closure that:
        // * takes no arguments (`||`)
        // * takes ownership of its captured variables (`move`)
        // * returns an unsigned 32-bit integer (`-> u32`)
        //
        // Rust is smart enough to infer the `-> u32` from
        // the closure itself so we could have left that out.
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                         // iterate over the characters of our segment..
                         .chars()
                         // .. convert text-characters to their number value..
                         .map(|c| c.to_digit(10).expect("should be a digit"))
                         // .. and sum the resulting iterator of numbers
                         .sum();
            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language",
            // the last evaluated expression in each block is 
            // automatically its value
            result
        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // Combine each thread's intermediate results into a single final sum.
    // We use the "turbofish" ::<> to provide sum() with a type hint.
    let final_result = children
                       // the iterator returned may yield any of `T`, `&T` or `&mut T`
                       .into_iter()
                       // ..wait for the thread to finish
                       .map(|c| c.join().unwrap())
                       // .. call sum on the returned values
                       .sum::<u32>();
    println!("Final sum result: {}", final_result);

                        

}
