use crate::array::Array;

impl Array {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if target > nums[mid] {
                if nums[mid] > nums[right] {
                    left = mid + 1;
                } else {
                    if target < nums[right] {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }   
                }
            } else if target < nums[mid] {
                if nums[mid] > nums[right] {
                    if target > nums[right] {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                } else {
                    right = mid;
                }
            } else {
                return mid as i32;
            }
        }

        if nums[left] == target { left as i32 } else { -1 }
    }
}

// 2, 4, 5, 6, 7, 0, 1
// 4, 5, 6, 7, 0, 1, 2
// 5, 6, 7, 0, 1, 2, 4
// 6, 7, 0, 1, 2, 4, 5