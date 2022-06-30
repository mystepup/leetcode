use super::DynamicProgramming;

impl DynamicProgramming {
    pub fn rob2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp = (0, 0);
        for n in nums.iter().take(nums.len() - 1) {
            dp = (dp.1, std::cmp::max(dp.0 + n, dp.1));
        }

        let max = std::cmp::max(dp.0, dp.1);
        dp = (0, 0);

        for n in nums.iter().skip(1) {
            dp = (dp.1, std::cmp::max(dp.0 + n, dp.1));
        }

        std::cmp::max(max, std::cmp::max(dp.0, dp.1))
    }
}