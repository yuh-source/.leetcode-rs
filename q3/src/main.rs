// 3. Longest Substring Without Repeating Characters

// Given a string s, find the length of the longest substring without duplicate characters.

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.

// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.

// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

// Constraints:

//     0 <= s.length <= 5 * 104
//     s consists of English letters, digits, symbols and spaces.

// time comp: O(n), 0ms
// space comp: O(128) = O(1)

struct Solution {

}

impl Solution {
    // sliding window approach
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        // size 128 array stores all english ASCII chars
        let mut valid_index = [0; 128];
        let mut start = 0;

        for (end, ch) in s.chars().enumerate() {
            // jumps start to next valid pos for that char
            start = start.max(valid_index[ch as usize]);
            max_length = max_length.max(end - start + 1);
            // next valid pos is 1 after last seen
            valid_index[ch as usize] = end + 1;
        }

        max_length as i32
    }
}

fn main() {
    let answer = Solution::length_of_longest_substring("jabcjdbefghibj".to_string());
    println!("{answer:?}");
}