use crate::binary::Binary;

impl Binary {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut input_bit: [u32; 32] = [0; 32];
        let mut x = x;
        let mut idx = 0;
        
        while x > 0 {
            if x % 2 == 1 {
                input_bit[idx] = 1;
            } else {
                input_bit[idx] = 0;
            }
            x = x / 2;
            idx += 1;
        }

        let mut result = 0;

        for i in (0..=31).rev() {
            result += input_bit[31 - i] * u32::pow(2, i as u32);
        }

        result
    }
}


// 13
// 6   1
// 3   0
// 1   1
// 0   1