// use leetcode::Array;
// use leetcode::Binary;
use leetcode::DynamicProgramming;

fn main() {
    // let result = Array::two_sum(vec![1, 2, 3], 4);
    // println!("{:?}", result);

    // let result = Binary::get_sum(1, 3);
    // println!("{:?}", result);

    let result = DynamicProgramming::climb_stairs(4);
    println!("{}", result);
}
