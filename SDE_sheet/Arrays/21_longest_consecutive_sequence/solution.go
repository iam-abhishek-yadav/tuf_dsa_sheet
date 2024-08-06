func longestConsecutive(nums []int) int {
    numSet := make(map[int]struct{})
    for _, num := range nums {
        numSet[num] = struct{}{}
    }

    maxLength := 0
    for num := range numSet {
        if _, found := numSet[num-1]; !found {
            curr := 1
            nextNum := num + 1
            for _, found := numSet[nextNum]; found; _, found = numSet[nextNum] {
                curr++
                nextNum++
            }
            if curr > maxLength {
                maxLength = curr
            }
        }
    }
    return maxLength
}
