struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        longest_palindrome_str(&s).to_string()
    }
}

fn longest_palindrome_str(s: &str) -> &str {
    if s.is_empty() {
        return "";
    }
    let mut longest_str = &s[0..1];
    for middle_index in 0..s.len() - 1 {
        let max_size = middle_index.max(s.len() - middle_index - 1);
        for even_offset in 0..max_size {
            let candidate_len = 2 * even_offset + 2;
            let begin_index = middle_index - even_offset;
            let ending_index = middle_index + even_offset + 1;
            // TODO: Make a unicode compaitble version
            if s.as_bytes()[begin_index] == s.as_bytes()[ending_index] {
                if candidate_len > longest_str.len() {
                    longest_str = &s[begin_index..ending_index + 1]
                }
            } else {
                break;
            }
        }

        for odd_offset in 0..max_size {
            let candidate_len = 2 * odd_offset + 3;
            let begin_index = middle_index - odd_offset;
            let ending_index = middle_index + odd_offset + 2;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::longest_palindrome("babad".to_string());
        assert!(
            result == "bab" || result == "aba",
            "Expected 'bab' or 'aba', but got '{}'",
            result
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test_single_char() {
        let result = Solution::longest_palindrome("a".to_string());
        assert_eq!(result, "a");
    }

    #[test]
    #[should_panic]
    fn test_empty_string_panics() {
        Solution::longest_palindrome("".to_string());
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_palindrome("aaaa".to_string()), "aaaa");
    }

    #[test]
    fn test_no_palindrome() {
        let result = Solution::longest_palindrome("abcde".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_long_odd_palindrome() {
        let result = Solution::longest_palindrome("racecar".to_string());
        assert_eq!(result, "racecar");
    }

    #[test]
    fn test_long_even_palindrome() {
        assert_eq!(Solution::longest_palindrome("abccba".to_string()), "abccba");
    }
}

