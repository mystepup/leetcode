mod binary;
use binary::Binary;

fn main() {
    let result = Binary::reverse_bits(2^32-3);
    println!("{:#032b}", 2^32-3);
    println!("{:#032b}", result);
}
