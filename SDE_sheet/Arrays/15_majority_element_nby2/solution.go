func majorityElement(nums []int) int {
    candidate := 0
    count := 0

    for _, num := range nums {
        if count == 0 {
            candidate = num
            count = 1
        } else if candidate == num {
            count++
        } else {
            count--
        }
    }

    count = 0
    for _, num := range nums {
        if candidate == num {
            count++
        }
    }

    if count > len(nums)/2 {
        return candidate
    }
    return -1
}
