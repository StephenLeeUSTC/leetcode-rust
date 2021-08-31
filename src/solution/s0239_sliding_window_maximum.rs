/**
 * [239] Sliding Window Maximum
 *
 * You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.
 * Return the max sliding window.
 *  
 * Example 1:
 * 
 * Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation: 
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 * 
 * Example 2:
 * 
 * Input: nums = [1], k = 1
 * Output: [1]
 * 
 * Example 3:
 * 
 * Input: nums = [1,-1], k = 1
 * Output: [1,-1]
 * 
 * Example 4:
 * 
 * Input: nums = [9,11], k = 2
 * Output: [11]
 * 
 * Example 5:
 * 
 * Input: nums = [4,-2], k = 2
 * Output: [4]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 	1 <= k <= nums.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-window-maximum/
// discuss: https://leetcode.com/problems/sliding-window-maximum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut index : VecDeque<usize> = VecDeque::new();
        let mut res : Vec<i32> = vec![];
        for i in 0..nums.len() {
            while (index.is_empty() == false && nums[i] > nums[*index.back().unwrap()]) {
                index.pop_back();
            }
            index.push_back(i);
            if (i >= (k as usize - 1)) {
                res.push(nums[*index.front().unwrap()]);
                // println!("i is {}, index is {:?}", i, index);
            }
            if (i - *index.front().unwrap() + 1 >= k as usize) {
                index.pop_front();
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
    fn test_239_0() {
        assert_eq!(Solution::max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
    }

    #[test]
    fn test_239_1() {
        assert_eq!(Solution::max_sliding_window(vec![1,-1], 1), vec![1, -1]);
    }

    #[test]
    fn test_239_2() {
        assert_eq!(Solution::max_sliding_window(vec![1,3, 1, 2, 0 ,5], 3), vec![3, 3, 2, 5]);
    }
}
