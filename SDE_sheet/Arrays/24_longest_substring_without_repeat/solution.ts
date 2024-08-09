function lengthOfLongestSubstring(s: string): number {
    let left: number = 0;
    let max: number = 0;
    const set: Set<string> = new Set();

    for (let right: number = 0; right < s.length; right++) {
        const char: string = s[right];

        while (set.has(char)) {
            set.delete(s[left]);
            left++;
        }

        set.add(char);
        max = Math.max(max, right - left + 1);
    }

    return max;
}

