fn unique_paths(m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![0; n]; m];
    
    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }
    
    dp[m - 1][n - 1]
}

