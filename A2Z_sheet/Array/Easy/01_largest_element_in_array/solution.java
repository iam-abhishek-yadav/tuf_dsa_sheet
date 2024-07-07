class Solution {
    public static int largest(int n, int[] arr) {
        int max = 0;
        for(int num: arr){
            max = Math.max(max, num);
        }
        return max;
    }
}
