use std::collections::HashMap;

pub struct Array {}

pub struct Binary {}

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