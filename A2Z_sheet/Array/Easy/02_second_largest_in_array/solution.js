class Solution {
    print2largest(arr) {
        let [max, secMax] = [-1, -1];
        for (const num of arr) {
            if (num > max) [secMax, max] = [max, num];
            else if (num > secMax && num < max) secMax = num;
        }
        return secMax;
    }
}
