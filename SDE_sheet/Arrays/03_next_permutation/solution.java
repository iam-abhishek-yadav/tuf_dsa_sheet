class Solution {
    public void nextPermutation(int[] nums) {
        int n = nums.length, index = n - 2;
        
        while (index >= 0 && nums[index] >= nums[index + 1]) {
            index--;
        }
        
        if (index >= 0) {
            int j = n - 1;
            while (j > index && nums[j] <= nums[index]) {
                j--;
            }
            int temp = nums[index];
            nums[index] = nums[j];
            nums[j] = temp;
        }
        reverse(nums, index + 1, n - 1);
    }
    public void reverse(int[] nums, int start, int end) {
        while (start < end) {
            int temp = nums[start];
            nums[start++] = nums[end];
            nums[end--] = temp;
        }
    }
}
