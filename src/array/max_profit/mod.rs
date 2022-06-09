use crate::array::Array;

impl Array {
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
}