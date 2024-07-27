func searchMatrix(matrix [][]int, target int) bool {
    m, n := len(matrix), len(matrix[0])
    left, right := 0, m*n-1

    for left <= right {
        mid := (left + right) / 2
        row, col := mid/n, mid%n
        if matrix[row][col] == target {
            return true
        }
        if matrix[row][col] > target {
            right = mid - 1
        } else {
            left = mid + 1
        }
    }

    return false
}
