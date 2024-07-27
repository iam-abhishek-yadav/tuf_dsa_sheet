fn number_of_inversions(a: &mut [i32]) -> i32 {
    merge_sort(a, 0, a.len() as isize - 1)
}

fn merge_sort(arr: &mut [i32], low: isize, high: isize) -> i32 {
    let mut cnt = 0;

    if low >= high {
        return cnt;
    }

    let mid = low + (high - low) / 2;

    cnt += merge_sort(arr, low, mid);
    cnt += merge_sort(arr, mid + 1, high);
    cnt += merge(arr, low, mid, high);

    cnt
}

fn merge(arr: &mut [i32], low: isize, mid: isize, high: isize) -> i32 {
    let mut temp = Vec::with_capacity((high - low + 1) as usize);
    let mut left = low;
    let mut right = mid + 1;
    let mut cnt = 0;

    while left <= mid && right <= high {
        if arr[left as usize] <= arr[right as usize] {
            temp.push(arr[left as usize]);
            left += 1;
        } else {
            temp.push(arr[right as usize]);
            cnt += (mid - left + 1) as i32;
            right += 1;
        }
    }

    while left <= mid {
        temp.push(arr[left as usize]);
        left += 1;
    }

    while right <= high {
        temp.push(arr[right as usize]);
        right += 1;
    }

    for i in low..=high {
        arr[i as usize] = temp[(i - low) as usize];
    }

    cnt
}

