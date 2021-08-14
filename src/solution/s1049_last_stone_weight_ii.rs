/**
 * [1049] Last Stone Weight II
 *
 * You are given an array of integers stones where stones[i] is the weight of the i^th stone.
 * We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights x and y with x <= y. The result of this smash is:
 * 
 * 	If x == y, both stones are destroyed, and
 * 	If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
 * 
 * At the end of the game, there is at most one stone left.
 * Return the smallest possible weight of the left stone. If there are no stones left, return 0.
 *  
 * Example 1:
 * 
 * Input: stones = [2,7,4,1,8,1]
 * Output: 1
 * Explanation:
 * We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
 * we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
 * we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
 * we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
 * 
 * Example 2:
 * 
 * Input: stones = [31,26,33,21,40]
 * Output: 5
 * 
 * Example 3:
 * 
 * Input: stones = [1,2]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= stones.length <= 30
 * 	1 <= stones[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/last-stone-weight-ii/
// discuss: https://leetcode.com/problems/last-stone-weight-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::max;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let hw: i32 = stones.iter().sum::<i32>() / 2;
        let mut dp = vec![vec![0; hw as usize + 1]; stones.len() + 1];
        // dp[i][j] means 0..i element, total weight is j;
        
        for i in 1..=stones.len() as usize {
            for j in 1..=hw as usize {
                if stones[i - 1] > j as i32 {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - stones[i - 1] as usize] + stones[i - 1]);
                }
            }
        }
        return stones.iter().sum::<i32>() - dp[stones.len()][hw as usize] * 2;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1049() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2,7,4,1,8,1]), 1);
        assert_eq!(Solution::last_stone_weight_ii(vec![31,26,33,21,40]), 5);
    }
}
