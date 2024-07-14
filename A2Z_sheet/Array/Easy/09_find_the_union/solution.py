class Solution:
    def findUnion(self,arr1,arr2,n,m):
        set_union = set()
        for num in arr1:
            set_union.add(num)
        for num in arr2:
            set_union.add(num)
        union_list = sorted(set_union)
        return union_list
