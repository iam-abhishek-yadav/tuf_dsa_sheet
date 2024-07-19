impl Solution {
    pub fn max_profit(mut prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        
        let mut max_profit = 0;

        for i in 1..prices.len() {
            if prices[i] < prices[0] {
                prices[0] = prices[i];
            } else {
                let profit = prices[i] - prices[0];
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }

        max_profit
    }
}

