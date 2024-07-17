class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        n = len(nums)
        maxim = float('-inf')
        curr = 0
        
        for num in nums:
            curr = max(curr + num, num)
            maxim = max(maxim, curr)
        
        return maxim

