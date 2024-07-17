class Solution {
    public int maxSubArray(int[] nums) {
        int n = nums.length, max = Integer.MIN_VALUE, curr = 0;
        for (int num : nums) {
            curr = Math.max(curr + num, num);
            max = Math.max(max, curr);
        }
        return max;
    }
}
