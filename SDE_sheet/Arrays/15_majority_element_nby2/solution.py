class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        candidate = None
        count = 0

        for num in nums:
            if count == 0:
                candidate = num
                count = 1
            elif candidate == num:
                count += 1
            else:
                count -= 1

        count = 0
        for num in nums:
            if candidate == num:
                count += 1

        return candidate if count > len(nums) / 2 else -1
