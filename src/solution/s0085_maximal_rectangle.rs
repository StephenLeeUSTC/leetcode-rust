/**
 * [85] Maximal Rectangle
 *
 * Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg" style="width: 402px; height: 322px;" />
 * Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 6
 * Explanation: The maximal rectangle is shown in the above picture.
 * 
 * Example 2:
 * 
 * Input: matrix = []
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: matrix = [["0"]]
 * Output: 0
 * 
 * Example 4:
 * 
 * Input: matrix = [["1"]]
 * Output: 1
 * 
 * Example 5:
 * 
 * Input: matrix = [["0","0"]]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	rows == matrix.length
 * 	cols == matrix[i].length
 * 	0 <= row, cols <= 200
 * 	matrix[i][j] is '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-rectangle/
// discuss: https://leetcode.com/problems/maximal-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        // dp[i][j] is the 1s end in col j in row i
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();

        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '0' {
                    dp[i][j] = 0;
                } else {
                    if i == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i - 1][j] + 1;
                    }
                }
            }
        }

        fn largest_rectangle_in_his(heights: &Vec<i32>) -> i32 {
            let size = heights.len();
            let mut stack:Vec<usize> = Vec::new();
            let mut res = 0;

            let mut height = 0;
            for i in 0..=size {
               height = if i < size {
                   heights[i]
               } else {
                   0
               };

               while stack.is_empty() == false && height < heights[*stack.last().unwrap()] {
                  let index = stack.pop().unwrap(); 

                  if stack.is_empty() {
                      res = std::cmp::max(res, heights[index] * i as i32);
                  } else {
                      res = std::cmp::max(res, heights[index] * (i - 1 - *stack.last().unwrap()) as i32);
                  }
               }
               stack.push(i);
            }

            res
        }

        let mut res = 0;

        for i in 0..m {
            res = std::cmp::max(res, largest_rectangle_in_his(&dp[i]));
        }
        
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_85_0() {
        let input = vec![vec!['1','0','1','0','0'], vec!['1','0','1','1','1'], vec!['1','1','1','1','1'], vec!['1','0','0','1','0']];
        assert_eq!(Solution::maximal_rectangle(input), 6);
    }

    #[test]
    fn test_85_1() {
        let input = vec![vec!['0']];
        assert_eq!(Solution::maximal_rectangle(input), 0);
    }

    #[test]
    fn test_85_2() {
        let input = vec![vec!['1']];
        assert_eq!(Solution::maximal_rectangle(input), 1);
    }
}
