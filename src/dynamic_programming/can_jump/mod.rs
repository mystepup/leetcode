use super::DynamicProgramming;

impl DynamicProgramming {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];

        for  i in 1..nums.len() {
            if dp[i - 1] < i as i32 {
                dp[i] = 0;
            } else {
                dp[i] = if nums[i] + (i as i32) > dp[i - 1] { nums[i] + (i as i32) } else { dp[i - 1] };
            }
        }

        dp[nums.len() - 1] as usize >= nums.len() - 1
    }
}