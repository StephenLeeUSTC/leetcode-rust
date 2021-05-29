/**
 * [461] Hamming Distance
 *
 * The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
 * Given two integers x and y, return the Hamming distance between them.
 *  
 * Example 1:
 * 
 * Input: x = 1, y = 4
 * Output: 2
 * Explanation:
 * 1   (0 0 0 1)
 * 4   (0 1 0 0)
 *        &uarr;   &uarr;
 * The above arrows point to positions where the corresponding bits are different.
 * 
 * Example 2:
 * 
 * Input: x = 3, y = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	0 <= x, y <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/hamming-distance/
// discuss: https://leetcode.com/problems/hamming-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut count = 0;
        while z != 0 {
            z = z & (z - 1);
            count += 1;
        }
        return count;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_461() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
