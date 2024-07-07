class Solution {
    public boolean check(int[] nums) {
        int count = 0;
        for(int i=1;i<nums.length;i++){
            if(nums[i] < nums[i-1]) count++;
        }
        if(count > 1) return false;
        return count == 0 ? true : nums[0] >= nums[nums.length-1];
    }
}
