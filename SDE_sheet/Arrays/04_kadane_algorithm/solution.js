var maxSubArray = function(nums) {
    let n = nums.length,max = Number.MIN_SAFE_INTEGER, curr = 0
    for(const num of nums){
        curr = Math.max(curr + num, num)
        max = Math.max(max, curr)
    }
    return max
};
