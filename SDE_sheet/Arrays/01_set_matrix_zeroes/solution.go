func setZeroes(matrix [][]int)  {
    n, m, flag := len(matrix), len(matrix[0]), 1;
    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            if (matrix[i][j] == 0) {
                matrix[i][0] = 0;
                if j != 0 {
                    matrix[0][j] = 0;
                } else {
                    flag = 0;
                }
            }
        }
    }
    for i := 1; i < n; i++ {
        for j := 1; j < m; j++ {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }
    if (matrix[0][0] == 0) {
        for i := 0; i < m; i++ {
            matrix[0][i] = 0;
        }
    }
    if (flag == 0) {
        for i := 0; i < n; i++ {
            matrix[i][0] = 0;
        }
    }
}
