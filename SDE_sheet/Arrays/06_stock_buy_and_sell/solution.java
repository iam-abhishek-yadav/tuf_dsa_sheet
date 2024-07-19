class Solution {
    public int maxProfit(int[] prices) {
        if (prices.length == 0) return 0;
        int maxProfit = 0;
        for (int i = 1; i < prices.length; i++) {
            if (prices[i] < prices[0]) {
                prices[0] = prices[i];
            } else {
                maxProfit = Math.max(maxProfit, prices[i] - prices[0]);
            }
        }
        return maxProfit;
    }
}
