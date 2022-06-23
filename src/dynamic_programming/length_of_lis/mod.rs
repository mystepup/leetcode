use std::vec;

use crate::dynamic_programming::DynamicProgramming;

impl DynamicProgramming {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        dp.push(nums[0]);

        for i in 1..nums.len() {
            match dp.binary_search(&nums[i]) {
                Ok(n) => (),
                Err(n) => {
                    if n >= dp.len() { dp.push(nums[i]) } else { dp[n] = nums[i] }
                }   
            }
        }

        dp.len() as i32
    }
}

// 10, 9, 2, 5, 3, 7, 101, 18


