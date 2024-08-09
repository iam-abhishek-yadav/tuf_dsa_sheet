impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left: usize = 0;
        let mut max_len: usize = 0;
        let mut char_map = std::collections::HashMap::new();

        let s_chars: Vec<char> = s.chars().collect();

        for (right, char) in s_chars.iter().enumerate() {
            if let Some(&index) = char_map.get(char) {
                if index >= left {
                    left = index + 1;
                }
            }
            char_map.insert(char, right);
            max_len = std::cmp::max(max_len, right - left + 1);
        }

        max_len as i32
    }
}

