func lengthOfLongestSubstring(s string) int {
    left := 0
    max := 0
    set := make(map[rune]int)

    for right, char := range s {
        if index, exists := set[char]; exists && index >= left {
            left = index + 1
        }
        set[char] = right
        if length := right - left + 1; length > max {
            max = length
        }
    }

    return max
}
