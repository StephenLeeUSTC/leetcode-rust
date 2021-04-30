/**
 * [403] Frog Jump
 *
 * A frog is crossing a river. The river is divided into some number of units, and at each unit, there may or may not exist a stone. The frog can jump on a stone, but it must not jump into the water.
 * Given a list of stones' positions (in units) in sorted ascending order, determine if the frog can cross the river by landing on the last stone. Initially, the frog is on the first stone and assumes the first jump must be 1 unit.
 * If the frog's last jump was k units, its next jump must be either k - 1, k, or k + 1 units. The frog can only jump in the forward direction.
 *  
 * Example 1:
 * 
 * Input: stones = [0,1,3,5,6,8,12,17]
 * Output: true
 * Explanation: The frog can jump to the last stone by jumping 1 unit to the 2nd stone, then 2 units to the 3rd stone, then 2 units to the 4th stone, then 3 units to the 6th stone, 4 units to the 7th stone, and 5 units to the 8th stone.
 * 
 * Example 2:
 * 
 * Input: stones = [0,1,2,3,4,8,9,11]
 * Output: false
 * Explanation: There is no way to jump to the last stone as the gap between the 5th and 6th stone is too large.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= stones.length <= 2000
 * 	0 <= stones[i] <= 2^31 - 1
 * 	stones[0] == 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/frog-jump/
// discuss: https://leetcode.com/problems/frog-jump/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        // dp[i][k] means index is i and move to i, the step is k
        // the worst case is 0, 1, 3, 6, 10, 15, 21;
        let s = stones.len();
        for i in 1..s {
            if stones[i] - stones[i - 1] > i as i32 {
                return false;
            }
        }

        let l = *stones.last().unwrap() as usize + 1;
        let mut dp = vec![vec![false; l]; s];
        dp[0][0] = true;

        for i in 0..s {
            for j in i + 1..s {
                let distance = stones[j] as usize - stones[i] as usize;
                if distance > i + 1 {
                    break;
                }
                if dp[i][distance] || dp[i][distance - 1] || dp[i][distance + 1] {
                    if j == s - 1 {
                        return true;
                    }
                    dp[j][distance] = true;
                }

            }
        }
        
        return false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_403() {
        assert_eq!(Solution::can_cross(vec![0,1,3,5,6,8,12,17]), true);
        assert_eq!(Solution::can_cross(vec![0,1,2,3,4,8,9,11]), false);
        assert_eq!(Solution::can_cross(vec![0,2,3,4]), false);
        assert_eq!(Solution::can_cross(vec![0,223123]), false);
    }
}
