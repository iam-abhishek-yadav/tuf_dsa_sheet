impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut v: Vec<i64> = nums.iter().map(|&n| n as i64).collect();
        v.sort();
        v.dedup();

        let n = v.len();
        let m = Self::pow_2(n);

        Self::extend(m, &mut v);

        let mut t = vec![0; 2 * m];
        let mut s = 0;

        for &num in &nums {
            let j = v.partition_point(|&n| n <= 2 * num as i64);
            let c = if j < v.len() { Self::count(j, v.len() - 1, m, &t) } else { 0 };

            let k = v.binary_search(&(num as i64)).unwrap();
            Self::add(k, m, 1, &mut t);

            s += c;
        }

        s
    }

    pub fn count(mut i: usize, mut j: usize, m: usize, t: &[i32]) -> i32 {
        i += m;
        j += m;

        let mut r = 0;

        while i <= j {
            if i % 2 == 1 {
                r += t[i];
                i += 1;
            }
            if j % 2 == 0 {
                r += t[j];
                j -= 1;
            }
            i /= 2;
            j /= 2;
        }

        r
    }

    pub fn add(mut i: usize, m: usize, x: i32, t: &mut Vec<i32>) {
        i += m;
        t[i] += x;

        while i > 1 {
            i /= 2;
            t[i] = t[2 * i] + t[2 * i + 1];
        }
    }

    pub fn pow_2(n: usize) -> usize {
        let mut r = 1;
        while r < n {
            r <<= 1;
        }
        r
    }

    pub fn extend(m: usize, v: &mut Vec<i64>) {
        let len = v.len();
        while v.len() < m {
            v.push(v.last().unwrap() + 1);
        }
    }
}

