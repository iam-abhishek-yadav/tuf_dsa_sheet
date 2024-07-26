function merge(nums1, m, nums2, n) {
    let idx1 = m - 1, idx2 = n - 1, mergedIdx = m + n - 1;
    while (idx1 >= 0 && idx2 >= 0) {
        nums1[mergedIdx--] = nums1[idx1] > nums2[idx2] ? nums1[idx1--] : nums2[idx2--];
    }
    while (idx2 >= 0) {
        nums1[mergedIdx--] = nums2[idx2--];
    }
}

