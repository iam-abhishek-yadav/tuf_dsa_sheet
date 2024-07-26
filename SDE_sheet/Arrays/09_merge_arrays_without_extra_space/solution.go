func merge(nums1 []int, m int, nums2 []int, n int)  {
    idx1, idx2, mergedIdx := m-1, n-1, m+n-1
    for idx1 >= 0 && idx2 >= 0 {
        if nums1[idx1] > nums2[idx2] {
            nums1[mergedIdx] = nums1[idx1]
            idx1--
        } else {
            nums1[mergedIdx] = nums2[idx2]
            idx2--
        }
        mergedIdx--
    }
    for idx2 >= 0 {
        nums1[mergedIdx] = nums2[idx2]
        idx2--
        mergedIdx--
    }
}
