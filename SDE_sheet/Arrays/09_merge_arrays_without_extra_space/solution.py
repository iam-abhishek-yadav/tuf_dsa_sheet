class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        idx1, idx2, merged_idx = m - 1, n - 1, m + n - 1
        while idx1 >= 0 and idx2 >= 0:
            nums1[merged_idx] = nums1[idx1] if nums1[idx1] > nums2[idx2] else nums2[idx2]
            if nums1[merged_idx] == nums1[idx1]:
                idx1 -= 1
            else:
                idx2 -= 1
            merged_idx -= 1
        while idx2 >= 0:
            nums1[merged_idx] = nums2[idx2]
            idx2 -= 1
            merged_idx -= 1
        
