class Solution{
    static int searchInSorted(int arr[], int N, int K){
        for (int num: arr){
            if(num == K) return 1;
        }
        return -1;
    }
}
