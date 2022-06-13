use crate::Array;

impl Array {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
        
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left]
    }
}


// 11, 13, 15, 17
// 5, 1, 2, 3, 4
// 2, 3, 4, 5, 1