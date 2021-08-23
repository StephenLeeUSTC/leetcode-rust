/**
 * [1646] Get Maximum in Generated Array
 *
 * You are given an integer n. An array nums of length n + 1 is generated in the following way:
 * 
 * 	nums[0] = 0
 * 	nums[1] = 1
 * 	nums[2 * i] = nums[i] when 2 <= 2 * i <= n
 * 	nums[2 * i + 1] = nums[i] + nums[i + 1] when 2 <= 2 * i + 1 <= n
 * 
 * Return the maximum integer in the array nums​​​.
 *  
 * Example 1:
 * 
 * Input: n = 7
 * Output: 3
 * Explanation: According to the given rules:
 *   nums[0] = 0
 *   nums[1] = 1
 *   nums[(1 * 2) = 2] = nums[1] = 1
 *   nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
 *   nums[(2 * 2) = 4] = nums[2] = 1
 *   nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
 *   nums[(3 * 2) = 6] = nums[3] = 2
 *   nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
 * Hence, nums = [0,1,1,2,1,3,2,3], and the maximum is 3.
 * 
 * Example 2:
 * 
 * Input: n = 2
 * Output: 1
 * Explanation: According to the given rules, the maximum between nums[0], nums[1], and nums[2] is 1.
 * 
 * Example 3:
 * 
 * Input: n = 3
 * Output: 2
 * Explanation: According to the given rules, the maximum between nums[0], nums[1], nums[2], and nums[3] is 2.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= n <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/get-maximum-in-generated-array/
// discuss: https://leetcode.com/problems/get-maximum-in-generated-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let mut arr = vec![0; n as usize + 1];
        arr[0] = 0;
        arr[1] = 1;
        let mut res = 0;
        for i in 2..=n as usize {
            if i % 2 == 1 {
                arr[i] = arr[(i - 1) / 2] + arr[(i - 1) / 2 + 1];
            } else {
                arr[i] = arr[i / 2];
            }
            res = std::cmp::max(res, arr[i]);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1646() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
        assert_eq!(Solution::get_maximum_generated(3), 2);
    }
}
