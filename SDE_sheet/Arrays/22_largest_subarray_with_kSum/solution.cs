public class Solution{
    public int maxLen(int[] arr, int k){
        int sum = 0, max = 0;
        Dictionary<int, int> map = new Dictionary<int, int>();
        map[0] = -1;
        
        for (int i = 0; i < k; i++){
            sum += arr[i];
            if (map.ContainsKey(sum)){
                max = Math.Max(max, i - map[sum]);
            }
            if (!map.ContainsKey(sum)){
                map[sum] = i;
            }
        }
        
        return max;
    }
}
