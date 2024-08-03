func reversePairs(nums []int) int {
    return mergeSortAndCount(nums, 0, len(nums)-1)
}

func mergeSortAndCount(nums []int, startIndex, endIndex int) int {
    if startIndex < endIndex {
        midIndex := (endIndex-startIndex)/2 + startIndex
        count := mergeSortAndCount(nums, startIndex, midIndex) +
                 mergeSortAndCount(nums, midIndex+1, endIndex)
        index1, index2 := startIndex, midIndex+1

        for index1 <= midIndex && index2 <= endIndex {
            if nums[index1] > 2*nums[index2] {
                count += midIndex - index1 + 1
                index2++
            } else {
                index1++
            }
        }

        merge(nums, startIndex, midIndex, endIndex)
        return count
    }
    return 0
}

func merge(nums []int, startIndex, midIndex, endIndex int) {
    length := endIndex - startIndex + 1
    newArray := make([]int, length)
    index1, index2 := startIndex, midIndex+1
    index := 0

    for index1 <= midIndex && index2 <= endIndex {
        if nums[index1] <= nums[index2] {
            newArray[index] = nums[index1]
            index1++
        } else {
            newArray[index] = nums[index2]
            index2++
        }
        index++
    }

    for index1 <= midIndex {
        newArray[index] = nums[index1]
        index1++
        index++
    }

    for index2 <= endIndex {
        newArray[index] = nums[index2]
        index2++
        index++
    }

    copy(nums[startIndex:startIndex+length], newArray)
}

