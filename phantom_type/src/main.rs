use std::marker::PhantomData;

// A phantom tuple struct which
// is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]
struct PhantomTuple<A,B>(A, PhantomData<B>);

// A phantom type struct which is
// generic over `A` with hidden parameter `B`
struct PhantomStruct<A, B> { 
    first: A, 
    phantom: PhantomData<B>,
}

// Note: Storage is allocated for generic type `A`,
// but not for `B`.
// Therefore, `B` cannot be used in computations.
fn main() {
    let _tuple1: PhantomTuple<char, f32> =
                 PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f32> =
                 PhantomTuple('Q', PhantomData);
   // Type specified as `<char, f32>`.
   let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
}
