class Solution {
    searchInSorted(arr, N, K){
        for (const num of arr){
            if(num === K) return 1;
        }
        return -1;
    }
}
