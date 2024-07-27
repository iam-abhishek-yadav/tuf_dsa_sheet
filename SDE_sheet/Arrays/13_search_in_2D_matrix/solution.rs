impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let (mut left, mut right) = (0, (m * n) as isize - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let row = (mid / n as isize) as usize;
            let col = (mid % n as isize) as usize;
            if matrix[row][col] == target {
                return true;
            }
            if matrix[row][col] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        false
    }
}
