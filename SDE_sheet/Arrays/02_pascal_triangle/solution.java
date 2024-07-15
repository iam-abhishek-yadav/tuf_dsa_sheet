class Solution {
    public List<List<Integer>> generate(int numRows) {
        List<List<Integer>> nums = new ArrayList<>();

        for (int i = 0; i < numRows; i++) {
            List<Integer> temp = new ArrayList<>();

            for (int j = 0; j <= i; j++) {
                if (j == 0 || j == i) {
                    temp.add(1);
                } else {
                    List<Integer> prevRow = nums.get(i - 1);
                    int left = prevRow.get(j - 1);
                    int right = prevRow.get(j);
                    temp.add(left + right);
                }
            }

            nums.add(temp);
        }

        return nums;
    }
}
