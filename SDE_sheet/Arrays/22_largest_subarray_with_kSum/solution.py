class Solution:
    def maxLen(self, n, arr):
        sum = 0
        max_len = 0
        map = {}
        
        for i, num in enumerate(arr):
            sum += num
            if sum == 0:
                max_len = i + 1
            if sum in map:
                max_len = max(max_len, i - map[sum])
            if sum not in map:
                map[sum] = i
        
        return max_len
