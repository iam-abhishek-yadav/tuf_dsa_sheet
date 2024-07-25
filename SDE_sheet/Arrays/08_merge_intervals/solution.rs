impl Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        let mut result = vec![intervals[0].clone()];

        for interval in intervals.iter() {
            let last_end = result.last().unwrap()[1];
            if interval[0] <= last_end {
                let last = result.last_mut().unwrap();
                last[1] = std::cmp::max(last_end, interval[1]);
            } else {
                result.push(interval.clone());
            }
        }

        result
    }
}
