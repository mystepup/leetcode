use std::collections::HashMap;

pub struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();

        for i in 0..(len-1) {
            for j in (i+1)..len {
                if target == nums[i] + nums[j] {
                    return vec![i as i32, j as i32];
                }
            }
        }

        return vec![0, 0];
    }
}