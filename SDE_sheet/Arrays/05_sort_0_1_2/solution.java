class Solution {
    public void sortColors(int[] nums) {
        int low = 0;
        int mid = 0;
        int high = nums.length - 1;

        while (mid <= high) {
            switch(nums[mid]) {
                case 0:
                    int tempLow = nums[low];
                    nums[low] = nums[mid];
                    nums[mid] = tempLow;
                    low++;
                    mid++;
                    break;
                case 1:
                    mid++;
                    break;
                case 2:
                    int tempHigh = nums[high];
                    nums[high] = nums[mid];
                    nums[mid] = tempHigh;
                    high--;
                    break;
            }
        }
    }
}

