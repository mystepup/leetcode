use std::collections::HashMap;
use crate::array::Array;

impl Array {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n = nums.len();
        let mut num_to_idx = HashMap::<i32, usize>::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let expected_sum = target - num;
            match num_to_idx.get(&expected_sum) {
                Some(&prev_idx) => return vec![prev_idx as i32, idx as i32],
                _ => {
                    num_to_idx.insert(num, idx);
                }
            }
        }

        unreachable!()
    }
}