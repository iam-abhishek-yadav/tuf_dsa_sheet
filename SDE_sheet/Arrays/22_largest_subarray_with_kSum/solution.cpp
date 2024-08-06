class Solution {
  public:
    int maxLen(vector<int>& arr, int n) {
       int sum = 0, max = 0;
        std::unordered_map<int, int> map;
        
        for (int i = 0; i < arr.size(); ++i) {
            sum += arr[i];
            if (sum == 0) {
                max = i + 1;
            }
            if (map.find(sum) != map.end()) {
                max = std::max(max, i - map[sum]);
            }
            if (map.find(sum) == map.end()) {
                map[sum] = i;
            }
        }
        return max;
    }
};
