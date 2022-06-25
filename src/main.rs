mod dynamic_programming;
use dynamic_programming::DynamicProgramming;

fn main() {
    let result = DynamicProgramming::word_break("leetcode".to_string(), vec!["leet".to_string(),"code".to_string()]);

    println!("{}", result);
}
