function removeDuplicates(nums: number[]): number {
    let k: number = 0;
    for(let i: number =1;i<nums.length;i++){
        if(nums[i] != nums[k]) nums[++k] = nums[i];
    }
    return k + 1;
};
