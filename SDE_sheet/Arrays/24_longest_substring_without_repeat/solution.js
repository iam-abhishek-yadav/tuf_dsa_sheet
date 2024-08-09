var lengthOfLongestSubstring = function(s) {
    let left = 0;
    let max = 0;
    const set = new Set();

    for (let right = 0; right < s.length; right++) {
        const char = s[right];

        while (set.has(char)) {
            set.delete(s[left]);
            left++;
        }

        set.add(char);
        max = Math.max(max, right - left + 1);
    }

    return max;
};
