mod array;
use array::Array;

// mod binary;
// use binary::Binary;

// mod dynamic_programming;
// use dynamic_programming::DynamicProgramming;

fn main() {
    let result = Array::max_product(vec![2,3,-2,4]);
    println!("{}", result);
}
