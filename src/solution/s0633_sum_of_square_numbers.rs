
/**
 * [633] Sum of Square Numbers
 *
 * Given a non-negative integer c, decide whether there're two integers a and b such that a^2 + b^2 = c.
 *  
 * Example 1:
 * 
 * Input: c = 5
 * Output: true
 * Explanation: 1 * 1 + 2 * 2 = 5
 * 
 * Example 2:
 * 
 * Input: c = 3
 * Output: false
 * 
 * Example 3:
 * 
 * Input: c = 4
 * Output: true
 * 
 * Example 4:
 * 
 * Input: c = 2
 * Output: true
 * 
 * Example 5:
 * 
 * Input: c = 1
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	0 <= c <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-square-numbers/
// discuss: https://leetcode.com/problems/sum-of-square-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i64;

        while left <= right {
            let sum : i64 = left * left + right * right;
            if sum == c as i64 {
                return true;
            } else if sum > c as i64 {
                right -= 1;
            } else {
                left += 1;
            }
        }
        return false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_633() {
        assert_eq!(Solution::judge_square_sum(5), true);
        assert_eq!(Solution::judge_square_sum(3), false);
        assert_eq!(Solution::judge_square_sum(4), true);
        assert_eq!(Solution::judge_square_sum(2), true);
        assert_eq!(Solution::judge_square_sum(1), true);
        assert_eq!(Solution::judge_square_sum(2147482647), false);
    }
}
