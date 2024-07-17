var nextPermutation = function(nums) {
    const n = nums.length;
    let index = n - 2;
    
    while (index >= 0 && nums[index] >= nums[index + 1]) {
        index--;
    }
    
    if (index >= 0) {
        let j = n - 1;
        while (j > index && nums[j] <= nums[index]) {
            j--;
        }
        [nums[index], nums[j]] = [nums[j], nums[index]];
    }
    reverse(nums, index + 1, n - 1);
};

var reverse = function(nums, start, end) {
    while (start < end) {
        [nums[start], nums[end]] = [nums[end], nums[start]];
        start++;
        end--;
    }
}
