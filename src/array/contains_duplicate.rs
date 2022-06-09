use std::collections::HashMap;
use crate::array::Array;

impl Array {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_to_count = HashMap::<i32, usize>::new();
        for (_, num) in nums.into_iter().enumerate() {
            match num_to_count.get(&num) {
                Some(&key) => {
                    if key == 0 {
                        return true;
                    }
                }
                None => {
                    num_to_count.insert(num, 0);
                }
            }
        }
    
        return false;
    }
}

