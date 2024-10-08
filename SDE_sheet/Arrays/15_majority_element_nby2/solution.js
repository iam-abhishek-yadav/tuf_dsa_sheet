var majorityElement = function(nums) {
    let candidate;
    let count = 0;

    for (const num of nums) {
        if (count === 0) {
            candidate = num;
            count = 1;
        } else if (candidate === num) {
            count++;
        } else {
            count--;
        }
    }

    count = 0;
    for (const num of nums) {
        if (candidate === num) count++;
    }

    return count > nums.length / 2 ? candidate : -1;
};
