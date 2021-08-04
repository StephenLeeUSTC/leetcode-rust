/**
 * [221] Maximal Square
 *
 * Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg" style="width: 400px; height: 319px;" />
 * Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 4
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg" style="width: 165px; height: 165px;" />
 * Input: matrix = [["0","1"],["1","0"]]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: matrix = [["0"]]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 300
 * 	matrix[i][j] is '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-square/
// discuss: https://leetcode.com/problems/maximal-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::min;
use std::cmp::max;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut res = i32::MIN;
        for i in 1..=m {
            for j in 1..=n {
                if matrix[i - 1][j - 1] == '0' {
                    dp[i][j] = 0;
                } else {
                    // println!("the value to compare is {}, {}, {}", dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]);
                    dp[i][j] = min(min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]) + 1;
                    // println!("dp[{}][{}] is {}", i, j, dp[i][j]);
                    res = max(res, dp[i][j]);
                }
            }
        }
        res * res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221_0() {
        let matrix = vec![vec!['0','1'],vec!['1','0']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }

    #[test]
    fn test_221_1() {
        let matrix = vec![vec!['0','1']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }


   #[test]
    fn test_221_2() {
        let source = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]];
        let m = source.len();
        let n = source[0].len();
        let mut matrix = vec![ vec![' '; source[0].len()]; source.len()];
        for i in 0..m {
            for j in 0..n {
                matrix[i][j] = source[i][j].chars().collect::<Vec<char>>()[0];
            }
        }
        
        assert_eq!(Solution::maximal_square(matrix), 4);
    }
}
