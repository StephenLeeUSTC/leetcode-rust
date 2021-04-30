/**
 * [137] Single Number II
 *
 * Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
 *  
 * Example 1:
 * Input: nums = [2,2,3,2]
 * Output: 3
 * Example 2:
 * Input: nums = [0,1,0,1,0,1,99]
 * Output: 99
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	Each element in nums appears exactly three times except for one element which appears once.
 * 
 *  
 * Follow up: Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-ii/
// discuss: https://leetcode.com/problems/single-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut total = 0;
            for num in nums.iter() {
                total += ((num >> i) & 1);
            }
            if total % 3 == 1 {
                res |= (1 << i);
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
    fn test_137() {
        assert_eq!(Solution::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution::single_number(vec![0,1,0,1,0,1,88]), 88);
    }
}
