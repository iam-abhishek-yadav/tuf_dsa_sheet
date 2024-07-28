impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n == i32::MIN {
            return Solution::my_pow(x, n + 1) * x;
        }
        let mut x = x;
        let mut n = n;
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        if n % 2 == 0 {
            Solution::my_pow(x * x, n / 2)
        } else {
            x * Solution::my_pow(x * x, (n - 1) / 2)
        }
    }
}
