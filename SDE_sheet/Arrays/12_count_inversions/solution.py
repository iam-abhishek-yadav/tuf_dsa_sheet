def number_of_inversions(a):
    return merge_sort(a, 0, len(a) - 1)

def merge_sort(arr, low, high):
    cnt = 0

    if low >= high:
        return cnt

    mid = low + (high - low) // 2

    cnt += merge_sort(arr, low, mid)
    cnt += merge_sort(arr, mid + 1, high)
    cnt += merge(arr, low, mid, high)

    return cnt

def merge(arr, low, mid, high):
    temp = []
    left = low
    right = mid + 1
    cnt = 0

    while left <= mid and right <= high:
        if arr[left] <= arr[right]:
            temp.append(arr[left])
            left += 1
        else:
            temp.append(arr[right])
            cnt += mid - left + 1
            right += 1

    while left <= mid:
        temp.append(arr[left])
        left += 1

    while right <= high:
        temp.append(arr[right])
        right += 1

    for i in range(low, high + 1):
        arr[i] = temp[i - low]

    return cnt

