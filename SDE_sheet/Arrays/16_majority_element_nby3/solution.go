func majorityElement(nums []int) []int {
    cnt1, cnt2 := 0, 0
    ele1, ele2 := int(^uint(0)>>1), int(^uint(0)>>1)

    for _, num := range nums {
        if cnt1 == 0 && num != ele2 {
            cnt1 = 1
            ele1 = num
        } else if cnt2 == 0 && num != ele1 {
            cnt2 = 1
            ele2 = num
        } else if num == ele1 {
            cnt1++
        } else if num == ele2 {
            cnt2++
        } else {
            cnt1--
            cnt2--
        }
    }

    cnt1, cnt2 = 0, 0

    for _, num := range nums {
        if num == ele1 {
            cnt1++
        }
        if num == ele2 {
            cnt2++
        }
    }

    req := len(nums) / 3
    result := []int{}

    if cnt1 > req {
        result = append(result, ele1)
    }
    if cnt2 > req {
        result = append(result, ele2)
    }

    return result
}
