mod binary;
use binary::Binary;

fn main() {
    let result = Binary::missing_number(vec![3, 0 ,2]);

    println!("{}", result);
}
