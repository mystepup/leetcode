mod binary;
use binary::Binary;

fn main() {
    let result = Binary::hammingWeight(4);

    println!("{}", result);
}
