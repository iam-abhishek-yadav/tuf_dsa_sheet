func check(nums []int) bool {
    count := 0
    for i := 1; i<len(nums); i++ {
        if(nums[i] < nums[i-1]){
            count++
        }
    }
    if count > 1 {
        return false
    }
    return count == 0 || nums[0] >= nums[len(nums) - 1]
}
