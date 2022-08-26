fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

} // box1 is destroyed here and memory gets freed



fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope
    {
        let _box3 = Box::new(4i32);
    }

    // creating lots of boxes just for fun
    for _ in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop!");

    // stack allocated integer
    let x = 5u32;

    // copy x into y - no resources are moved
    let y = x;
    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

}
