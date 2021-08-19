/**
 * [16] 3Sum Closest
 *
 * Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
 * Return the sum of the three integers.
 * You may assume that each input would have exactly one solution.
 *  
 * Example 1:
 * 
 * Input: nums = [-1,2,1,-4], target = 1
 * Output: 2
 * Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 * 
 * Example 2:
 * 
 * Input: nums = [0,0,0], target = 1
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	3 <= nums.length <= 1000
 * 	-1000 <= nums[i] <= 1000
 * 	-10^4 <= target <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-closest/
// discuss: https://leetcode.com/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut inputs = nums.clone();
        inputs.sort();
        // println!("inputs is {:?}", inputs);
        let mut res = 0;
        let mut diff = std::i32::MAX;

        for i in 0..inputs.len() {
            let mut j = i + 1;
            let mut k = inputs.len() - 1;
            let mut sum = 0;
            while j < k {
                sum = inputs[i] + inputs[j] + inputs[k];
                // println!("i, j , k is {}, {}, {}, sum is {}", i, j, k, sum);
                if sum == target {
                    return sum;
                } else if sum < target {
                    if i32::abs(sum - target) < diff {
                        diff = i32::abs(sum - target);
                        res = sum;
                    }
                    j += 1;
                } else {
                    if i32::abs(sum - target) < diff {
                        diff = i32::abs(sum - target);
                        res = sum;
                    }
                    k -= 1;
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
    fn test_16_0() {
        let nums = vec![-1,2,1,-4];
        let target = 1;
        assert_eq!(Solution::three_sum_closest(nums, target), 2);
    }

    #[test]
    fn test_16_1() {
        let nums = vec![0, 0, 0];
        let target = 1;
        assert_eq!(Solution::three_sum_closest(nums, target), 0);
    }

    #[test]
    fn test_16_3() {
        let nums = vec![0, 1, 2];
        let target = 3;
        assert_eq!(Solution::three_sum_closest(nums, target), 3);
    }

    #[test]
    fn test_16_4() {
        let nums = vec![1, 1, 1, 0];
        let target = -100;
        assert_eq!(Solution::three_sum_closest(nums, target), 2);
    }
}
