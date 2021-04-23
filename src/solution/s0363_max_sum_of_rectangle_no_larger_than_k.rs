/**
 * [363] Max Sum of Rectangle No Larger Than K
 *
 * Given an m x n matrix matrix and an integer k, return the max sum of a rectangle in the matrix such that its sum is no larger than k.
 * It is guaranteed that there will be a rectangle with a sum no larger than k.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/18/sum-grid.jpg" style="width: 255px; height: 176px;" />
 * Input: matrix = [[1,0,1],[0,-2,3]], k = 2
 * Output: 2
 * Explanation: Because the sum of the blue rectangle [[0, 1], [-2, 3]] is 2, and 2 is the max number no larger than k (k = 2).
 * 
 * Example 2:
 * 
 * Input: matrix = [[2,2,-1]], k = 3
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 100
 * 	-100 <= matrix[i][j] <= 100
 * 	-10^5 <= k <= 10^5
 * 
 *  
 * Follow up: What if the number of rows is much larger than the number of columns?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// discuss: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i == 0 && j == 0 {
                    dp[i][j] = matrix[i][j];
                } else if i == 0 && j != 0 {
                    dp[i][j] = dp[i][j - 1] + matrix[i][j];
                } else if i != 0 && j == 0 {
                    dp[i][j] = dp[i - 1][j] + matrix[i][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1] + matrix[i][j] - dp[i - 1][j - 1];
                }
            }
        }
        // println!("{:?}", dp);
        let mut res = i32::MIN;
        let mut tmp = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                for ii in 0..=i {
                    for jj in 0..=j {
                        if ii == 0 && jj == 0 {
                            tmp = dp[i][j];
                        } else if ii == 0 && jj != 0 {
                            tmp = dp[i][j] - dp[i][jj - 1];
                        } else if ii != 0 && jj == 0 {
                            tmp = dp[i][j] - dp[ii - 1][j];
                        } else {
                            tmp = dp[i][j] - dp[ii - 1][j] - dp[i][jj - 1] + dp[ii - 1][jj - 1];
                        }
                        if tmp <= k {
                            res = std::cmp::max(res, tmp)
                        }
                    }
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_363() {
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2,2,-1]], 3), 3);
        assert_eq!(Solution::max_sum_submatrix(vec![vec![1,0,1], vec![0,-2,3]], 2), 2);
    }
}
