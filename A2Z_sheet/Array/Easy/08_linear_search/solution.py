class Solution:
    def searchInSorted(self,arr, N, K):
        for i in range(N):
            if arr[i] == K:
                return 1
        return -1
