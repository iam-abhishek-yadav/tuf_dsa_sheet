impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0;
        let mut mid = 0;
        let mut high = nums.len() as i32 - 1;

        while mid as i32 <= high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                },
                1 => {
                    mid += 1;
                },
                2 => {
                    nums.swap(mid, high as usize);
                    high -= 1;
                },
                _ => {}
            }
        }
    }
}
