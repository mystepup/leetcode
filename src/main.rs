mod dynamic_programming;
use dynamic_programming::DynamicProgramming;

fn main() {
    let result = DynamicProgramming::num_decodings("123".to_string());

    println!("{}", result)
}
