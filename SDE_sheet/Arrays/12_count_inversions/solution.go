func numberOfInversions(a []int) int {
    return mergeSort(a, 0, len(a)-1)
}

func mergeSort(arr []int, low, high int) int {
    cnt := 0

    if low >= high {
        return cnt
    }

    mid := low + (high-low)/2

    cnt += mergeSort(arr, low, mid)
    cnt += mergeSort(arr, mid+1, high)
    cnt += merge(arr, low, mid, high)

    return cnt
}

func merge(arr []int, low, mid, high int) int {
    temp := make([]int, 0, high-low+1)
    left := low
    right := mid + 1
    cnt := 0

    for left <= mid && right <= high {
        if arr[left] <= arr[right] {
            temp = append(temp, arr[left])
            left++
        } else {
            temp = append(temp, arr[right])
            cnt += mid - left + 1
            right++
        }
    }

    for left <= mid {
        temp = append(temp, arr[left])
        left++
    }

    for right <= high {
        temp = append(temp, arr[right])
        right++
    }

    for i := low; i <= high; i++ {
        arr[i] = temp[i-low]
    }

    return cnt
}

