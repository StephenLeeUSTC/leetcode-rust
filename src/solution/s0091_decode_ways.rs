/**
 * [91] Decode Ways
 *
 * A message containing letters from A-Z can be encoded into numbers using the following mapping:
 * 
 * 'A' -> "1"
 * 'B' -> "2"
 * ...
 * 'Z' -> "26"
 * 
 * To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
 * 
 * 	"AAJF" with the grouping (1 1 10 6)
 * 	"KJF" with the grouping (11 10 6)
 * 
 * Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
 * Given a string s containing only digits, return the number of ways to decode it.
 * The answer is guaranteed to fit in a 32-bit integer.
 *  
 * Example 1:
 * 
 * Input: s = "12"
 * Output: 2
 * Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
 * 
 * Example 2:
 * 
 * Input: s = "226"
 * Output: 3
 * Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
 * 
 * Example 3:
 * 
 * Input: s = "0"
 * Output: 0
 * Explanation: There is no character that is mapped to a number starting with 0.
 * The only valid mappings with 0 are 'J' -> "10" and 'T' -> "20", neither of which start with 0.
 * Hence, there are no valid ways to decode this since all digits need to be mapped.
 * 
 * Example 4:
 * 
 * Input: s = "06"
 * Output: 0
 * Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 100
 * 	s contains only digits and may contain leading zero(s).
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-ways/
// discuss: https://leetcode.com/problems/decode-ways/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn get_value(sc: &Vec<i32>, left: usize, right: usize) -> i32 {
        let mut res = 0;
        for i in left..=right {
            res = res * 10 + sc[i];
        }
        return res;
    }
    pub fn num_decodings(s: String) -> i32 {
        let sc: Vec<i32> = s.chars().map(|c| { c as i32 - '0' as i32 }).collect::<Vec<_>>();
        // println!("sc is {:?}", sc);

        let mut dp: Vec<i32> = vec![0; sc.len()];
        if Self::check_legal(Self::get_value(&sc, 0, 0)) {
            dp[0] = 1;
        } else {
            return 0;
        }
        if dp.len() == 1 {
            return dp[0];
        }
        if Self::check_legal(Self::get_value(&sc, 1, 1)) {
            dp[1] += 1;
        }
        if Self::check_legal(Self::get_value(&sc, 0, 1)) {
            dp[1] += 1;
        }
        for i in 2..sc.len() {
            if Self::check_legal(Self::get_value(&sc, i, i)) {
                dp[i] += dp[i - 1];
            }
            if Self::check_legal(Self::get_value(&sc, i - 1, i)) && Self::check_legal(Self::get_value(&sc, i - 1, i - 1)) {
                dp[i] += dp[i - 2];
            }
        }
        return *dp.last().unwrap();
    }

    pub fn dfs(res: &mut i32, path: i32, s: &[u8], index: usize) {
        if Self::check_legal(path) {
            if index == s.len() {
                *res += 1;
                return;
            }
        } else {
            return;
        }

        // do choice
        let mut t_path = s[index] as i32 - '0' as i32;
        Self::dfs(res, t_path, s, index + 1);

        if t_path == 0 || index + 1 == s.len() {
            return;
        }
        t_path = t_path * 10 + (s[index + 1] as i32 - '0' as i32);
        Self::dfs(res, t_path, s, index + 2);
    }

    pub fn check_legal(path: i32) -> bool {
        return path >= 1 && path <= 26;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
        assert_eq!(Solution::num_decodings("066".to_string()), 0);
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
        assert_eq!(Solution::num_decodings("1".to_string()), 1);
        assert_eq!(Solution::num_decodings("106".to_string()), 1);
        assert_eq!(Solution::num_decodings("100".to_string()), 0);
        assert_eq!(Solution::num_decodings("27".to_string()), 1);
    }
}
