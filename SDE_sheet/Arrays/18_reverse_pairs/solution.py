class Solution:
    @staticmethod
    def reversePairs(nums):
        def mergeSortAndCount(nums, startIndex, endIndex):
            if startIndex >= endIndex:
                return 0

            midIndex = (startIndex + endIndex) // 2
            count = mergeSortAndCount(nums, startIndex, midIndex) + \
                    mergeSortAndCount(nums, midIndex + 1, endIndex)
            count += countReversePairs(nums, startIndex, midIndex, endIndex)

            merge(nums, startIndex, midIndex, endIndex)
            return count

        def countReversePairs(nums, startIndex, midIndex, endIndex):
            count = 0
            j = midIndex + 1
            for i in range(startIndex, midIndex + 1):
                while j <= endIndex and nums[i] > 2 * nums[j]:
                    j += 1
                count += j - (midIndex + 1)
            return count

        def merge(nums, startIndex, midIndex, endIndex):
            temp = nums[startIndex:endIndex + 1]
            i, j, k = startIndex, midIndex + 1, 0

            while i <= midIndex and j <= endIndex:
                if nums[i] <= nums[j]:
                    temp[k] = nums[i]
                    i += 1
                else:
                    temp[k] = nums[j]
                    j += 1
                k += 1

            while i <= midIndex:
                temp[k] = nums[i]
                i += 1
                k += 1

            while j <= endIndex:
                temp[k] = nums[j]
                j += 1
                k += 1

            nums[startIndex:endIndex + 1] = temp

        return mergeSortAndCount(nums, 0, len(nums) - 1)

