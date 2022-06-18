use crate::binary::Binary;

impl Binary {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0];

        for i in 1..=n {
            result.push(result[(i as usize) / 2] + i % 2);
        }

        result
    }
}