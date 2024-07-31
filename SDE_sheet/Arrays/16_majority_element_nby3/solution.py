class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        cnt1, cnt2 = 0, 0
        ele1, ele2 = float('inf'), float('inf')

        for num in nums:
            if cnt1 == 0 and num != ele2:
                cnt1 = 1
                ele1 = num
            elif cnt2 == 0 and num != ele1:
                cnt2 = 1
                ele2 = num
            elif num == ele1:
                cnt1 += 1
            elif num == ele2:
                cnt2 += 1
            else:
                cnt1 -= 1
                cnt2 -= 1

        cnt1, cnt2 = 0, 0

        for num in nums:
            if num == ele1:
                cnt1 += 1
            if num == ele2:
                cnt2 += 1

        req = len(nums) // 3
        result = []

        if cnt1 > req:
            result.append(ele1)
        if cnt2 > req:
            result.append(ele2)

        return result
