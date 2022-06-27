use super::DynamicProgramming;

impl DynamicProgramming {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; (target + 1) as usize];

        dp[0] = 1;

        for i in 1..dp.len() {
            for j in 0..nums.len() {
                if (i >= (nums[j] as usize)) {
                    dp[i] += dp[(i as usize) - nums[j]];
                }
            }
        }

        dp[dp.len() - 1]
    }
}