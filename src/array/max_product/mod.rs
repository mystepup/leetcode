use crate::array::Array;

impl Array {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = nums[0];
        let (mut imax, mut imin) = (max, max);

        for i in 1..n {
            if nums[i] < 0 {
                let tmp = imax;
                imax = imin;
                imin = tmp;
            }

            imax = if nums[i] > imax * nums[i] { nums[i] } else { imax * nums[i] };
            imin = if nums[i] > imin * nums[i] { imin * nums[i] } else { nums[i] };

            max = if max > imax { max } else { imax }
        }

        max
    }
}