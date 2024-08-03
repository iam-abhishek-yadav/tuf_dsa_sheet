function reversePairs(nums: number[]): number {
    return mergeSortAndCount(nums, 0, nums.length - 1);

    function mergeSortAndCount(nums: number[], startIndex: number, endIndex: number): number {
        if (startIndex < endIndex) {
            const midIndex = Math.floor((endIndex - startIndex) / 2) + startIndex;
            let count = mergeSortAndCount(nums, startIndex, midIndex) +
                        mergeSortAndCount(nums, midIndex + 1, endIndex);
            let index1 = startIndex, index2 = midIndex + 1;

            while (index1 <= midIndex && index2 <= endIndex) {
                if (nums[index1] > 2 * nums[index2]) {
                    count += midIndex - index1 + 1;
                    index2++;
                } else {
                    index1++;
                }
            }

            merge(nums, startIndex, midIndex, endIndex);
            return count;
        } else {
            return 0;
        }
    }

    function merge(nums: number[], startIndex: number, midIndex: number, endIndex: number): void {
        const length = endIndex - startIndex + 1;
        const newArray = new Array(length);
        let index1 = startIndex, index2 = midIndex + 1;
        let index = 0;

        while (index1 <= midIndex && index2 <= endIndex) {
            newArray[index++] = nums[index1] <= nums[index2] ? nums[index1++] : nums[index2++];
        }

        while (index1 <= midIndex) {
            newArray[index++] = nums[index1++];
        }

        while (index2 <= endIndex) {
            newArray[index++] = nums[index2++];
        }

        for (let i = 0; i < length; i++) {
            nums[startIndex + i] = newArray[i];
        }
    }
}

