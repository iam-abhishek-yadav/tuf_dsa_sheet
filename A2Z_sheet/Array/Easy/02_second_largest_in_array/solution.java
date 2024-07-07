class Solution {
    public int print2largest(List<Integer> arr) {
        int max = -1, secMax = -1;
        for(int num: arr){
            if(num > max){
                secMax = max;
                max = num;
            }
            else if(num > secMax && num < max) secMax = num;
        }
        return secMax;
    }
}
