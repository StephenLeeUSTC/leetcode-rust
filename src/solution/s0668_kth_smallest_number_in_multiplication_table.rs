/**
 * [668] Kth Smallest Number in Multiplication Table
 *
 * Nearly everyone has used the <a href="https://en.wikipedia.org/wiki/Multiplication_table" target="_blank">Multiplication Table</a>. The multiplication table of size m x n is an integer matrix mat where mat[i][j] == i * j (1-indexed).
 * Given three integers m, n, and k, return the k^th smallest element in the m x n multiplication table.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/multtable1-grid.jpg" style="width: 500px; height: 254px;" />
 * Input: m = 3, n = 3, k = 5
 * Output: 3
 * Explanation: The 5^th smallest number is 3.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/multtable2-grid.jpg" style="width: 493px; height: 293px;" />
 * Input: m = 2, n = 3, k = 6
 * Output: 6
 * Explanation: The 6^th smallest number is 6.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= m, n <= 3 * 10^4
 * 	1 <= k <= m * n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/
// discuss: https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(m: i32, n: i32, target: i32) -> i32 {
        // return the number of element less equal than target
        let mut count = 0;
        for i in 1..=m {
            count += std::cmp::min(target / i, n); 
        }

        count
    }

    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = m * n;
        let mut mid = 0;

        while left <= right {
            mid = (right - left) / 2 + left;
            let count = Self::helper(m, n, mid as i32);
            if count < k {
                left = mid + 1;
            } else if count > k {
                right = mid - 1;
            } else {
                right = mid - 1;
            }
        }

        return left;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_668() {
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }
}
