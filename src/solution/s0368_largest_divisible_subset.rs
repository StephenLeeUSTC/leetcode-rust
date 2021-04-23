/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
 * 
 * 	answer[i] % answer[j] == 0, or
 * 	answer[j] % answer[i] == 0
 * 
 * If there are multiple solutions, return any of them.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3]
 * Output: [1,2]
 * Explanation: [1,3] is also accepted.
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,4,8]
 * Output: [1,2,4,8]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 2 * 10^9
 * 	All the integers in nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-divisible-subset/
// discuss: https://leetcode.com/problems/largest-divisible-subset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut dp = vec![0; nums.len()];
        let mut parent = vec![0; nums.len()];
        for i in 0..parent.len() {
            parent[i] = i;
        }
        let mut input = nums.clone();
        // input.sort_by(|a , b| b.cmp(a));
        input.sort();
        // println!("input is {:?}", input);

        for i in 0..input.len() {
            for j in 0..i {
                if input[i] % input[j] == 0 && (dp[j] + 1 > dp[i]) { 
                    dp[i] =  dp[j] + 1;
                    parent[i] = j;
                }
            }
        }
        // println!("dp is {:?}", dp);
        // println!("parent is {:?}", parent);
        let mut index = 0;
        let mut max_value = dp[0];
        for (i, value) in dp.iter().enumerate() {
            if *value > max_value {
                index = i;
                max_value = *value;
            }
        }
        // println!("index is {}", index);
        // println!("max_value is {}", max_value);

        let mut res = vec![];
        for _ in 0..=max_value {
            res.push(input[index]);
            index = parent[index];
        }
        res.reverse();
        return res;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
        assert_eq!(Solution::largest_divisible_subset(vec![5, 3]), vec![3]);
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 4]), vec![1, 2, 4]);
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 3, 4]), vec![1, 2, 4]);
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 8, 4]), vec![1, 2, 4, 8]);
    }
}
