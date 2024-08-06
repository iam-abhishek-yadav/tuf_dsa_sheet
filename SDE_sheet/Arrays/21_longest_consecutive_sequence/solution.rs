use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut max_length = 0;
        
        for &num in num_set.iter() {
            if !num_set.contains(&(num - 1)) {
                let mut curr = 1;
                let mut next_num = num + 1;
                while num_set.contains(&next_num) {
                    curr += 1;
                    next_num += 1;
                }
                max_length = max_length.max(curr);
            }
        }
        
        max_length
    }
}
