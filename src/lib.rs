use std::collections::HashMap;

pub struct Array {}

pub struct Binary {}

pub struct DynamicProgramming {}

impl Array {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n = nums.len();
        let mut num_to_idx = HashMap::<i32, usize>::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let expected_sum = target - num;
            match num_to_idx.get(&expected_sum) {
                Some(&prev_idx) => return vec![prev_idx as i32, idx as i32],
                _ => {
                    num_to_idx.insert(num, idx);
                }
            }
        }

        unreachable!()
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut cur_profit = 0;
        for (_, price) in prices.into_iter().enumerate() {
            let profit = price - min;

            if profit > cur_profit {
                cur_profit = profit;
            }

            if min > price {
                min = price;
            }
        }

        return cur_profit;
    }

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

impl Binary {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut y = b;
        let mut x = a;
        let mut carry;

        while y != 0 {
            carry = x & y;
            x = x ^ y;
            y = carry << 1;
        }
        return x;
    }
}

impl DynamicProgramming {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1, 2];

        for i in 2..n {
            dp.push(dp[i - 1] + dp[i - 2]);
        }

        dp[n - 1]
    }
}