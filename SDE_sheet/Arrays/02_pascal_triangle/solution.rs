impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut nums: Vec<Vec<i32>> = Vec::new();
        
        if num_rows == 0 {
            return nums;
        }
        
        nums.push(vec![1]);
        
        for i in 1..num_rows as usize {
            let mut row: Vec<i32> = vec![0; i + 1];
            row[0] = 1;
            row[i] = 1;
            
            for j in 1..i {
                row[j] = nums[i - 1][j - 1] + nums[i - 1][j];
            }
            
            nums.push(row);
        }
        
        nums
    }
}
