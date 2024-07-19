class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if len(prices) == 0:
            return 0
        maxProfit = 0
        for i in range(1, len(prices)):
            if prices[i] < prices[0]:
                prices[0] = prices[i]
            else:
                maxProfit = max(maxProfit, prices[i] - prices[0])
        return maxProfit

