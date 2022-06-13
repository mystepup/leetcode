mod array;
use array::Array;

// mod binary;
// use binary::Binary;

// mod dynamic_programming;
// use dynamic_programming::DynamicProgramming;

fn main() {
    let result = Array::find_min(vec![1, 2]);
    println!("{}", result);
}
