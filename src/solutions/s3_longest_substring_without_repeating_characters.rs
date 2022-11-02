// https://leetcode.com/problems/longest-substring-without-repeating-characters/

// submission starts
use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen_chars = HashMap::with_capacity(s.len());
        let mut substr_start_index = 0;
        let mut longest = 0;
        for (index, ch) in s.char_indices() {
            seen_chars
                .entry(ch)
                .and_modify(|old_index| {
                    // current char has been seen before
                    if *old_index >= substr_start_index {
                        // 當前重複的字符，老index大於當前的子串起點，需要算下當前的最大後，增大起點
                        longest = max(longest, index - substr_start_index);
                        substr_start_index = *old_index + 1;
                    }
                    // 無論怎樣，再次遇到重複都需要重新更新seen index
                    *old_index = index;
                })
                .or_insert(index);
        }
        // 以防沒有遇到重複不進入modify的場景，需要再max一次
        max(longest, s.len() - substr_start_index) as i32
    }
}
// submission ends

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("abcdefg".to_string()),
            7
        );
    }
}
