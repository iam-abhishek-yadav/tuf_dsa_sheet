function maxProfit(prices: number[]): number {
    if (prices.length === 0) return 0;
    let maxProfit: number = 0;
    for (let i = 1; i < prices.length; i++) {
        if (prices[i] < prices[0]) {
            prices[0] = prices[i];
        } else {
            maxProfit = Math.max(maxProfit, prices[i] - prices[0]);
        }
    }
    return maxProfit;
}
