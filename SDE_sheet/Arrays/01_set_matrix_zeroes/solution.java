class Solution {
    public void setZeroes(int[][] matrix) {
        int n = matrix.length, m = matrix[0].length, flag = 1;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    if (j != 0) matrix[0][j] = 0;
                    else flag = 0;
                }
            }
        }
        for (int i = 1; i < n; i++) {
            for (int j = 1; j < m; j++) {
                matrix[i][j] = (matrix[i][0] == 0 || matrix[0][j] == 0) ? 0 : matrix[i][j];
            }
        }
        if (matrix[0][0] == 0) {
            for (int i = 0; i < m; i++) matrix[0][i] = 0;
        }
        if (flag == 0) {
            for (int i = 0; i < n; i++) matrix[i][0] = 0;
        }
    }
}
