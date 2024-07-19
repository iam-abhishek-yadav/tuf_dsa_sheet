func maxProfit(prices []int) int {
    if len(prices) == 0 {
        return 0
    }
    
    maxProfit := 0

    for i := 1; i < len(prices); i++ {
        if prices[i] < prices[0] {
            prices[0] = prices[i]
        } else {
            profit := prices[i] - prices[0]
            if profit > maxProfit {
                maxProfit = profit
            }
        }
    }

    return maxProfit
}

