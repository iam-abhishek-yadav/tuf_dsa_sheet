class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda x: x[0])
        res = []
        
        i = 0
        while i < len(intervals):
            start, end = intervals[i]
            
            while i + 1 < len(intervals) and end >= intervals[i + 1][0]:
                i += 1
                end = max(end, intervals[i][1])
            
            res.append([start, end])
            i += 1
        
        return res
