struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        longest_palindrome_str(&s).to_string()
    }
}

fn longest_palindrome_str(s: &str) -> &str {
    let mut longest_str = &s[0..0];
    for middle_index in 0..s.len() - 1 {
        let max_size = middle_index.max(s.len() - middle_index - 1);
        for offset in 0..max_size {
            let candidate_len = 2 * offset + 2;
            let begin_index = middle_index - offset;
            let ending_index = middle_index + offset + 1;
            // TODO: Make a unicode compaitble version
            if s.as_bytes()[begin_index] == s.as_bytes()[ending_index] {
                if candidate_len > longest_str.len() {
                    longest_str = &s[begin_index..ending_index + 1]
                }
            } else {
                break;
            }
        }
    }
    longest_str
}

// Example 1:
//
// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
//
// Example 2:
//
// Input: s = "cbbd"
// Output: "bb"
