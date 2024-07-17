func nextPermutation(nums []int) {
    var n int = len(nums)
    var index int = n - 2
    
    for index >= 0 && nums[index] >= nums[index + 1] {
        index--
    }
    
    if index >= 0 {
        var j int = n - 1
        for j > index && nums[j] <= nums[index] {
            j--
        }
        
        nums[index], nums[j] = nums[j], nums[index]
    }
    
    reverse(nums, index + 1, n - 1)
}

func reverse(nums []int, start int, end int) {
    for start < end {
        nums[start], nums[end] = nums[end], nums[start]
        start++
        end--
    }
}

