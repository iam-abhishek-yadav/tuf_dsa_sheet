class Solution {
    public int reversePairs(int[] nums) {
        return mergeSortAndCount(nums, 0, nums.length - 1);
    }

    private int mergeSortAndCount(int[] nums, int startIndex, int endIndex) {
        if (startIndex >= endIndex) return 0;

        int midIndex = (startIndex + endIndex) / 2;
        int count = mergeSortAndCount(nums, startIndex, midIndex) +
                    mergeSortAndCount(nums, midIndex + 1, endIndex);

        int j = midIndex + 1;
        for (int i = startIndex; i <= midIndex; i++) {
            while (j <= endIndex && (long)nums[i] > 2 * (long)nums[j]) {
                j++;
            }
            count += j - (midIndex + 1);
        }

        merge(nums, startIndex, midIndex, endIndex);
        return count;
    }

    private void merge(int[] nums, int startIndex, int midIndex, int endIndex) {
        int[] temp = new int[endIndex - startIndex + 1];
        int i = startIndex, j = midIndex + 1, k = 0;

        while (i <= midIndex && j <= endIndex) {
            if (nums[i] <= nums[j]) {
                temp[k++] = nums[i++];
            } else {
                temp[k++] = nums[j++];
            }
        }

        while (i <= midIndex) {
            temp[k++] = nums[i++];
        }

        while (j <= endIndex) {
            temp[k++] = nums[j++];
        }

        System.arraycopy(temp, 0, nums, startIndex, temp.length);
    }
}

