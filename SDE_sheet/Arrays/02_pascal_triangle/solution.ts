function generate(numRows: number): number[][] {
    const nums: number[][] = [];
    
    for (let i = 0; i < numRows; i++) {
        const temp: number[] = [];
        
        for (let j = 0; j <= i; j++) {
            if (j === 0 || j === i) {
                temp.push(1);
            } else {
                const prevRow = nums[i - 1];
                const left = prevRow[j - 1] || 0;
                const right = prevRow[j] || 0;
                temp.push(left + right);
            }
        }
        nums.push(temp);
    }
    return nums;
}
