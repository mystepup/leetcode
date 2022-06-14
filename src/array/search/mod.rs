use crate::array::Array;

impl Array {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
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

        let rot = left;
        left = 0;
        right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let real_mid = (mid + rot) % nums.len();

            if nums[real_mid] == target {
                return real_mid as i32;
            }

            if nums[real_mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

// 2, 4, 5, 6, 7, 0, 1
// 4, 5, 6, 7, 0, 1, 2
// 5, 6, 7, 0, 1, 2, 4
// 6, 7, 0, 1, 2, 4, 5