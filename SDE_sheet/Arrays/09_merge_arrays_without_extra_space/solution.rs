impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut idx1, mut idx2, mut merged_idx) = (m as isize - 1, n as isize - 1, (m + n) as isize - 1);
        while idx1 >= 0 && idx2 >= 0 {
            if nums1[idx1 as usize] > nums2[idx2 as usize] {
                nums1[merged_idx as usize] = nums1[idx1 as usize];
                idx1 -= 1;
            } else {
                nums1[merged_idx as usize] = nums2[idx2 as usize];
                idx2 -= 1;
            }
            merged_idx -= 1;
        }
        while idx2 >= 0 {
            nums1[merged_idx as usize] = nums2[idx2 as usize];
            idx2 -= 1;
            merged_idx -= 1;
        }
    }
}
