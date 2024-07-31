class Solution {
    public List<Integer> majorityElement(int[] nums) {
        int cnt1 = 0, cnt2 = 0;
        Integer ele1 = null, ele2 = null;

        for (int num : nums) {
            if (cnt1 == 0 && (ele2 == null || num != ele2)) {
                cnt1 = 1;
                ele1 = num;
            } else if (cnt2 == 0 && (ele1 == null || num != ele1)) {
                cnt2 = 1;
                ele2 = num;
            } else if (num == ele1) {
                cnt1++;
            } else if (num == ele2) {
                cnt2++;
            } else {
                cnt1--;
                cnt2--;
            }
        }

        cnt1 = 0;
        cnt2 = 0;

        for (int num : nums) {
            if (ele1 != null && num == ele1) cnt1++;
            if (ele2 != null && num == ele2) cnt2++;
        }

        List<Integer> result = new ArrayList<>();
        int req = nums.length / 3;
        if (ele1 != null && cnt1 > req) result.add(ele1);
        if (ele2 != null && cnt2 > req) result.add(ele2);

        return result;
    }
}
