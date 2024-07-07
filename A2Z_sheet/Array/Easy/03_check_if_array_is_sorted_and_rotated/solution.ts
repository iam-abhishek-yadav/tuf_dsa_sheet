function check(nums: number[]): boolean {
    let count: number = 0;
    for(let i: number=1;i<nums.length;i++){
        if(nums[i] < nums[i-1]) count++;
    }
    if(count > 1) return false;
    return count == 0 ? true : nums[0] >= nums[nums.length-1];
};
