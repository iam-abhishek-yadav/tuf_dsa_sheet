class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        nums = []

        for i in range(numRows):
            temp = []

            for j in range(i + 1):
                if j == 0 or j == i:
                    temp.append(1)
                else:
                    prevRow = nums[i - 1]
                    left = prevRow[j - 1] if j - 1 >= 0 else 0
                    right = prevRow[j] if j < len(prevRow) else 0
                    temp.append(left + right)

            nums.append(temp)

        return nums
