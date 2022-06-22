mod dynamic_programming;
use dynamic_programming::DynamicProgramming;

fn main() {
    let result = DynamicProgramming::coin_change(vec![1, 2, 5], 6);

    println!("{}", result);
}
