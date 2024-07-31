class Solution {
    public int majorityElement(int[] nums) {
        Integer candidate = null;
        int count = 0;

        for (int num : nums) {
            if (count == 0) {
                candidate = num;
                count = 1;
            } else if (candidate.equals(num)) {
                count++;
            } else {
                count--;
            }
        }

        count = 0;
        for (int num : nums) {
            if (candidate.equals(num)) count++;
        }

        return count > nums.length / 2 ? candidate : -1;
    }
}
