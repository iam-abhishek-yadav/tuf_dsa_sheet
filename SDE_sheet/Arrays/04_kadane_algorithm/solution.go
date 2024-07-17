func maxSubArray(nums []int) int {
    maxim := math.MinInt32
    curr := 0
    
    for _, num := range nums {
        curr = max(curr + num, num)
        maxim = max(maxim, curr)
    }
    
    return maxim
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
