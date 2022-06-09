use crate::binary::Binary;

impl Binary {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut y = b;
        let mut x = a;
        let mut carry;

        while y != 0 {
            carry = x & y;
            x = x ^ y;
            y = carry << 1;
        }
        return x;
    }
}