class Solution:
    def print2largest(self, arr):
        max, secMax = -1, -1
        for num in arr:
            if num > max:
                secMax = max;
                max = num
            elif num > secMax and num < max:
                secMax = num
        return secMax
