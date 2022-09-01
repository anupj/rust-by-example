// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> 
                            impl Iterator<Item=i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
} 

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    // use double_positives()
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
