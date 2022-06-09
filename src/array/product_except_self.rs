use crate::array::Array;

impl Array {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut total_mul = 1;
        let mut result = Vec::<i32>::new();
        let mut zero_count = 0;

        if nums.len() == 1 && nums[0] == 0 {
            result.push(0);
            return result;
        }

        let iter1 = nums.iter();
        let iter2 = nums.iter();

        for num in iter1 {
            if *num != 0 {
                total_mul *= *num;
            } else {
                zero_count += 1;
            }
        }
        
        if zero_count > 1 {
            for _ in 0..nums.len() {
                result.push(0);
            }
            return result;
        }

        for num in iter2 {
            if *num == 0 {
                result.push(total_mul);
            } else {
                if zero_count > 0 {
                    result.push(0);
                } else {
                    result.push(total_mul / (*num));
                }
                
            }
        }
        
        result
    }
}