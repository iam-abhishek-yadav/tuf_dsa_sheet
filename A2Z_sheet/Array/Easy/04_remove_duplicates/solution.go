func removeDuplicates(nums []int) int {
    k := 0
    for i := 1; i<len(nums); i++ {
        if(nums[i] != nums[k]){
            k += 1;
            nums[k] = nums[i];
        }
    }
    return k + 1;
}
