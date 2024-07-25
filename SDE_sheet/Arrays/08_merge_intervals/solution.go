func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	
	res := [][]int{}
	
	for i := 0; i < len(intervals); i++ {
		start, end := intervals[i][0], intervals[i][1]
		
		for i + 1 < len(intervals) && end >= intervals[i + 1][0] {
			i++
			end = max(end, intervals[i][1])
		}
		
		res = append(res, []int{start, end})
	}
	
	return res
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
