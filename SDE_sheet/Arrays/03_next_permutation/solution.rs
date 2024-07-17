impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        fn get_border(nums: &[i32]) -> Option<usize> {
            let n = nums.len();
            for idx in (0..n - 1).rev() {
                if nums[idx] < nums[idx + 1] {
                    return Some(idx);
                }
            }
            None
        }
        
        if let Some(mut idx) = get_border(nums) {
            let n = nums.len();
            
            for idx2 in (0..n).rev() {
                if nums[idx2] > nums[idx] {
                    nums.swap(idx, idx2);
                    break;
                }
            }
            
            nums[idx + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

