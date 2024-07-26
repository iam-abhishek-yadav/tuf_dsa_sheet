function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    let idx1: number = m - 1, idx2: number = n - 1, mergedIdx: number = m + n - 1;
    while (idx1 >= 0 && idx2 >= 0) {
        nums1[mergedIdx--] = nums1[idx1] > nums2[idx2] ? nums1[idx1--] : nums2[idx2--];
    }
    while (idx2 >= 0) {
        nums1[mergedIdx--] = nums2[idx2--];
    }
};
