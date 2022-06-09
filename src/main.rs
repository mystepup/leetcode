mod array;
use array::Array;

// mod binary;
// use binary::Binary;

// mod dynamic_programming;
// use dynamic_programming::DynamicProgramming;

fn main() {
    let nums = Array::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]);
    println!("{}", nums);
}
