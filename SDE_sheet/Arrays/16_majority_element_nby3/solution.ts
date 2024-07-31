function majorityElement(nums: number[]): number[] {
    let cnt1: number = 0;
    let cnt2: number = 0;
    let ele1: number | null = null;
    let ele2: number | null = null;

    for (const num of nums) {
        if (cnt1 === 0 && (ele2 === null || num !== ele2)) {
            cnt1 = 1;
            ele1 = num;
        } else if (cnt2 === 0 && (ele1 === null || num !== ele1)) {
            cnt2 = 1;
            ele2 = num;
        } else if (num === ele1) {
            cnt1++;
        } else if (num === ele2) {
            cnt2++;
        } else {
            cnt1--;
            cnt2--;
        }
    }

    cnt1 = 0;
    cnt2 = 0;

    for (const num of nums) {
        if (ele1 !== null && num === ele1) cnt1++;
        if (ele2 !== null && num === ele2) cnt2++;
    }

    const req: number = nums.length / 3;
    const result: number[] = [];

    if (ele1 !== null && cnt1 > req) result.push(ele1);
    if (ele2 !== null && cnt2 > req) result.push(ele2);

    return result;
}
