use std::mem;

// This fn borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements initialised to the same value
    let _ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same as above but more verbose

    // Arrays can be safely accessed using `.get`
    // which returns an Option
    for i in 0..xs.len() + 1 { // OOPS, one element too far
        match xs.get(i) {
            Some(arr_val) => println!("{}: {}", i, arr_val),
            None => println!("Slow down! {} is too far!", i),
        }

    }
}
