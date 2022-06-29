use super::DynamicProgramming;

impl DynamicProgramming {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];

        if n == 1 {
            return nums[0];
        }

        dp[0] = nums[0];
        dp[1] = if nums[0] > nums[1] { nums[0] } else { nums[1] };

        for i in 2..n {
            dp[i] = std::cmp::max(dp[i - 2] + nums[i], dp[i - 1]);
        }

        dp[n - 1]
    }
}