function maxSubArray(nums: number[]): number {
    let n: number = nums.length
    let max: number = Number.MIN_SAFE_INTEGER
    let curr: number = 0
    for(const num of nums){
        curr = Math.max(curr + num, num)
        max = Math.max(max, curr)
    }
    return max
};
