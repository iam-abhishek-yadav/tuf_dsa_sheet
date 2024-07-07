impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count: u8 = 0;
        for i in 1..nums.len() {
            if(nums[i] < nums[i-1]) {
                count += 1;
            }
        }
        if count > 1 {
            return false
        }
        return count == 0 || nums[0] >= nums[nums.len() - 1];
    }
}
