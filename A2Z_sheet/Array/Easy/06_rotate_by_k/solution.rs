impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;
        let k = k % n;
        Self::reverse(nums, 0, n - 1);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k, n - 1);
    }

    fn reverse(nums: &mut Vec<i32>, start: i32, end: i32) {
        let mut start = start;
        let mut end = end;
        while start < end {
            nums.swap(start as usize, end as usize);
            start += 1;
            end -= 1;
        }
    }
}