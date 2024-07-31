impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        let mut ele1 = i32::MAX;
        let mut ele2 = i32::MAX;

        for &num in &nums {
            if cnt1 == 0 && num != ele2 {
                cnt1 = 1;
                ele1 = num;
            } else if cnt2 == 0 && num != ele1 {
                cnt2 = 1;
                ele2 = num;
            } else if num == ele1 {
                cnt1 += 1;
            } else if num == ele2 {
                cnt2 += 1;
            } else {
                cnt1 -= 1;
                cnt2 -= 1;
            }
        }

        cnt1 = 0;
        cnt2 = 0;

        for &num in &nums {
            if num == ele1 {
                cnt1 += 1;
            }
            if num == ele2 {
                cnt2 += 1;
            }
        }

        let req = nums.len() / 3;
        let mut result = Vec::new();

        if cnt1 > req {
            result.push(ele1);
        }
        if cnt2 > req {
            result.push(ele2);
        }

        result
    }
}
