class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        n = len(nums)
        index = n - 2
        
        while index >= 0 and nums[index] >= nums[index + 1]:
            index -= 1
        
        if index >= 0:
            j = n - 1
            while j > index and nums[j] <= nums[index]:
                j -= 1
            nums[index], nums[j] = nums[j], nums[index]
        self.reverse(nums, index + 1, n - 1)

    def reverse(self, nums: List[int], start: int, end: int) -> None:
        while start < end:
            nums[start], nums[end] = nums[end], nums[start]
            start += 1
            end -= 1
