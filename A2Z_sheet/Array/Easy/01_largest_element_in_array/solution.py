class Solution:
    def largest(self, n : int, arr : List[int]) -> int:
        maxValue = 0
        for num in arr:
            maxValue = max(maxValue, num)
        return maxValue
