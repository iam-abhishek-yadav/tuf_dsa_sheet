function searchMatrix(matrix: number[][], target: number): boolean {
    const [m, n] = [matrix.length, matrix[0].length];
    let [left, right] = [0, m * n - 1];

    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        const row = Math.floor(mid / n);
        const col = mid % n;
        if (matrix[row][col] === target) return true;
        if (matrix[row][col] > target) right = mid - 1;
        else left = mid + 1;
    }

    return false;
}

