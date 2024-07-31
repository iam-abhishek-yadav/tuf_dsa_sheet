impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = None;
        let mut count = 0;

        for &num in &nums {
            if count == 0 {
                candidate = Some(num);
                count = 1;
            } else if candidate == Some(num) {
                count += 1;
            } else {
                count -= 1;
            }
        }

        let candidate = candidate.unwrap_or(-1);
        let mut count = 0;
        for &num in &nums {
            if num == candidate {
                count += 1;
            }
        }

        if count > nums.len() / 2 {
            candidate
        } else {
            -1
        }
    }
}
