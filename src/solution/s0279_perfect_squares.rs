/**
 * [279] Perfect Squares
 *
 * Given an integer n, return the least number of perfect square numbers that sum to n.
 * A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
 *  
 * Example 1:
 * 
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 * 
 * Example 2:
 * 
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^4
 * 
 */
pub struct Solution {}

 // discuss: https://leetcode.com/problems/perfect-squares discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::min;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut nums = vec![];
        for i in 1..n {
            if i * i > n {
                break;
            } else {
                nums.push(i * i);
            }
        }
        // println!("nums is {:?}", nums);
        let l = nums.len();
        // indeed a package problem
        // dp[i][j] = min(dp[i - 1][j - nums[i]] + 1) for nums[i] <= j
        let mut dp = vec![vec![n; n as usize + 1]; l + 1];
        for i in 0..=l as usize {
            dp[i][0] = 0;
        }

        for i in 1..=l as usize {
            for j in 1..=n as usize {
                if nums[i - 1] <= j as i32 {
                    dp[i][j] = min(dp[i][j - nums[i - 1] as usize] + 1, dp[i - 1][j]);
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
                // println!("dp[{}][{}] is {}", i, j, dp[i][j]);
            }
        }

        return dp[l as usize][n as usize];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_279() {
        assert_eq!(Solution::num_squares(9), 1);
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
