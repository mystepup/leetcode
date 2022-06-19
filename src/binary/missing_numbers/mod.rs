use crate::binary::Binary;

impl Binary {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum: i32 = nums.iter().sum();

        for i in 0..=n {
            sum -= i as i32;
        }

        -sum
    }
}