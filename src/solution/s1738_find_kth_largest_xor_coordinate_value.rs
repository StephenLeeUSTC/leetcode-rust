/**
 * [1738] Find Kth Largest XOR Coordinate Value
 *
 * You are given a 2D matrix of size m x n, consisting of non-negative integers. You are also given an integer k.
 * The value of coordinate (a, b) of the matrix is the XOR of all matrix[i][j] where 0 <= i <= a < m and 0 <= j <= b < n (0-indexed).
 * Find the k^th largest value (1-indexed) of all the coordinates of matrix.
 *  
 * Example 1:
 * 
 * Input: matrix = [[5,2],[1,6]], k = 1
 * Output: 7
 * Explanation: The value of coordinate (0,1) is 5 XOR 2 = 7, which is the largest value.
 * Example 2:
 * 
 * Input: matrix = [[5,2],[1,6]], k = 2
 * Output: 5
 * Explanation: The value of coordinate (0,0) is 5 = 5, which is the 2nd largest value.
 * Example 3:
 * 
 * Input: matrix = [[5,2],[1,6]], k = 3
 * Output: 4
 * Explanation: The value of coordinate (1,0) is 5 XOR 1 = 4, which is the 3rd largest value.
 * Example 4:
 * 
 * Input: matrix = [[5,2],[1,6]], k = 4
 * Output: 0
 * Explanation: The value of coordinate (1,1) is 5 XOR 2 XOR 1 XOR 6 = 0, which is the 4th largest value.
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 1000
 * 	0 <= matrix[i][j] <= 10^6
 * 	1 <= k <= m * n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-kth-largest-xor-coordinate-value/
// discuss: https://leetcode.com/problems/find-kth-largest-xor-coordinate-value/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut t = vec![vec![0; col + 1]; row + 1];
        let mut ship = vec![];
        for i in 1..=row {
            for j in 1..=col {
                t[i][j] = t[i - 1][j] ^ t[i][j - 1] ^ t[i - 1][j - 1] ^ matrix[i - 1][j - 1];
                ship.push(t[i][j]);
            }
        }

        ship.sort_by(|a, b| b.cmp(a));
        return ship[k as usize - 1];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1738() {
        /* 
         * Input: matrix = [[5,2],[1,6]], k = 1
         * Output: 7
         * Explanation: The value of coordinate (0,1) is 5 XOR 2 = 7, which is the largest value.
         */
        assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1), 7);
        assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2), 5);
        assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3), 4);
        assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 4), 0);
    }
}
