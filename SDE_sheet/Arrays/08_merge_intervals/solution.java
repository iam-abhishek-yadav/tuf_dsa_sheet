class Solution {
    public int[][] merge(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        List<int[]> res = new ArrayList<>();
        
        for (int i = 0; i < intervals.length; i++) {
            int start = intervals[i][0];
            int end = intervals[i][1];
            
            while (i + 1 < intervals.length && end >= intervals[i + 1][0]) {
                i++;
                end = Math.max(end, intervals[i][1]);
            }
            
            res.add(new int[]{start, end});
        }
        
        return res.toArray(new int[res.size()][]);
    }
}
