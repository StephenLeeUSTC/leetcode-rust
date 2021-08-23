/**
 * [977] Squares of a Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
 *  
 * Example 1:
 * 
 * Input: nums = [-4,-1,0,3,10]
 * Output: [0,1,9,16,100]
 * Explanation: After squaring, the array becomes [16,1,0,9,100].
 * After sorting, it becomes [0,1,9,16,100].
 * 
 * Example 2:
 * 
 * Input: nums = [-7,-3,2,3,11]
 * Output: [4,9,9,49,121]
 * 
 *  
 * Constraints:
 * 
 * 	<span>1 <= nums.length <= </span>10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums is sorted in non-decreasing order.
 * 
 *  
 * Follow up: Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/squares-of-a-sorted-array/
// discuss: https://leetcode.com/problems/squares-of-a-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut k = nums.len();

        while i < nums.len() && i <= j {
            k -= 1;
            if i32::abs(nums[i]) >= i32::abs(nums[j]) {
                res[k] = nums[i] * nums[i];
                i += 1;
            } else {
                res[k] = nums[j] * nums[j];
                j -= 1
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
    fn test_977() {
        let nums = vec![-4];
        let res = Solution::sorted_squares(nums);
        println!("res is {:?}", res);
    }
}
