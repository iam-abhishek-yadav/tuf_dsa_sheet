impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        
        for i in 0..n {
            for j in (i + 1)..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

