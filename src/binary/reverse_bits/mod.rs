use crate::binary::Binary;

impl Binary {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut res, mut x) = (0u32, x);
        for _ in 0..32 {
            res = (res << 1) | (x & 1);
            x >>= 1;
        }

        res
    }
}


// 13
// 6   1
// 3   0
// 1   1
// 0   1