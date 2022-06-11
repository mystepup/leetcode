use crate::array::Array;

impl Array {
    pub fn max_sub_array(nums: Vec<i32>) -> i32  {
        let len = nums.len();
        let mut dp = Vec::new();
        dp.push(nums[0]);
        let mut max = dp[0];

        for i in 1..len {
            dp.push(nums[i] + (if dp[i-1] > 0 { dp[i-1] } else { 0 }));
            max = if max > dp[i] { max } else { dp[i] };
        }
        println!("{:?}", dp);
        max 
    }
}