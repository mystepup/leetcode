use crate::Array;

impl Array {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort();

        let n = nums.len() as i32;
        for i in 0..n-2 {
            let i = i as usize;
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = (n - 1) as usize;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }

        res
    }
}