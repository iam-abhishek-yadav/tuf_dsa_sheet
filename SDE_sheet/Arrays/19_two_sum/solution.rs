use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_map.get(&complement) {
                return vec![i as i32, index as i32];
            }
            num_map.insert(num, i as i32);
        }
        vec![]
    }
}

