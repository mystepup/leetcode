use crate::dynamic_programming::DynamicProgramming;

impl DynamicProgramming {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut lis = 1;

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }

            lis = std::cmp::max(lis, dp[i]);
        }

        lis
    }
}

// 10, 9, 2, 5, 3, 7, 101, 18


