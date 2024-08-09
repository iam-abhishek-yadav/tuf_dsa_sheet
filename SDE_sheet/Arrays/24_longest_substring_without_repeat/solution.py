class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        left, res = 0, 0
        char_set = set()

        for right in range(len(s)):
            char = s[right]

            while char in char_set:
                char_set.remove(s[left])
                left += 1

            char_set.add(char)
            res = max(res, right - left + 1)

        return res

