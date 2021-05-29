/**
 * [1035] Uncrossed Lines
 *
 * We write the integers of nums1 and nums2 (in the order they are given) on two separate horizontal lines.
 * Now, we may draw connecting lines: a straight line connecting two numbers nums1[i] and nums2[j] such that:
 * 
 * 	nums1[i] == nums2[j];
 * 	The line we draw does not intersect any other connecting (non-horizontal) line.
 * 
 * Note that a connecting lines cannot intersect even at the endpoints: each number can only belong to one connecting line.
 * Return the maximum number of connecting lines we can draw in this way.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/04/26/142.png" style="width: 100px; height: 72px;" />
 * Input: nums1 = <span id="example-input-1-1">[1,4,2]</span>, nums2 = <span id="example-input-1-2">[1,2,4]</span>
 * Output: <span id="example-output-1">2</span>
 * Explanation: We can draw 2 uncrossed lines as in the diagram.
 * We cannot draw 3 uncrossed lines, because the line from nums1[1]=4 to nums2[2]=4 will intersect the line from nums1[2]=2 to nums2[1]=2.
 * 
 * <div>
 * Example 2:
 * 
 * Input: nums1 = <span id="example-input-2-1">[2,5,1,2,5]</span>, nums2 = <span id="example-input-2-2">[10,5,2,1,5,2]</span>
 * Output: <span id="example-output-2">3</span>
 * 
 * <div>
 * Example 3:
 * 
 * Input: nums1 = <span id="example-input-3-1">[1,3,7,1,7,5]</span>, nums2 = <span id="example-input-3-2">[1,9,2,5,1]</span>
 * Output: <span id="example-output-3">2</span>
 *  
 * </div>
 * </div>
 * Note:
 * <ol>
 * 	1 <= nums1.length <= 500
 * 	1 <= nums2.length <= 500
 * 	<font face="monospace">1 <= nums1[i], nums2[i] <= 2000</font>
 * </ol>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/uncrossed-lines/
// discuss: https://leetcode.com/problems/uncrossed-lines/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::max;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i in 1..=l1 {
            for j in 1..=l2 {
                if (nums1[i - 1] == nums2[j - 1]) {
                    dp[i][j] = max(dp[i][j], dp[i - 1][j - 1] + 1);
                } else {
                    dp[i][j] = max(dp[i][j], max(dp[i - 1][j], dp[i][j - 1]));
                }
            }
        }
        // println!("dp is {:?}", dp);
        dp[nums1.len()][nums2.len()]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1035() {
        assert_eq!(Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);

        // Input: nums1 = <span id="example-input-2-1">[2,5,1,2,5]</span>, nums2 = <span id="example-input-2-2">[10,5,2,1,5,2]</span>
        // Output: <span id="example-output-2">3</span>
        assert_eq!(Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]), 3);
        // * Input: nums1 = <span id="example-input-3-1">[1,3,7,1,7,5]</span>, nums2 = <span id="example-input-3-2">[1,9,2,5,1]</span>
        // * Output: <span id="example-output-3">2</span>
        assert_eq!(Solution::max_uncrossed_lines(vec![1,3,7,1,7,5], vec![1, 9, 2, 5, 1]), 2);
    }
}
