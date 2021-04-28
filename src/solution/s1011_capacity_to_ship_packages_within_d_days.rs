/**
 * [1011] Capacity To Ship Packages Within D Days
 *
 * A conveyor belt has packages that must be shipped from one port to another within D days.
 * The i^th package on the conveyor belt has a weight of weights[i]. Each day, we load the ship with packages on the conveyor belt (in the order given by weights). We may not load more weight than the maximum weight capacity of the ship.
 * Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within D days.
 *  
 * Example 1:
 * 
 * Input: weights = [1,2,3,4,5,6,7,8,9,10], D = 5
 * Output: 15
 * Explanation: A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
 * 1st day: 1, 2, 3, 4, 5
 * 2nd day: 6, 7
 * 3rd day: 8
 * 4th day: 9
 * 5th day: 10
 * Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.
 * 
 * Example 2:
 * 
 * Input: weights = [3,2,2,4,1,4], D = 3
 * Output: 6
 * Explanation: A ship capacity of 6 is the minimum to ship all the packages in 3 days like this:
 * 1st day: 3, 2
 * 2nd day: 2, 4
 * 3rd day: 1, 4
 * 
 * Example 3:
 * 
 * Input: weights = [1,2,3,1,1], D = 4
 * Output: 3
 * Explanation:
 * 1st day: 1
 * 2nd day: 2
 * 3rd day: 3
 * 4th day: 1, 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= D <= weights.length <= 5 * 10^4
 * 	1 <= weights[i] <= 500
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/
// discuss: https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut left = *weights.iter().max().unwrap();
        let mut right: i32 = weights.iter().sum();
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::check(&weights, d, mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }

    fn check(weight: &Vec<i32>, d: i32, cap: i32) -> bool {
        let mut need = 1;
        let mut sum = 0;
        for w in weight.iter() {
            if sum + *w > cap {
                need += 1;
                sum = *w;
            } else {
                sum += *w;
            }
        }
        if need <= d {
            // println!("d is {}, cap is {}, res is true", d, cap);
            true
        } else {
            // println!("d is {}, cap is {}, res is false", d, cap);
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1011() {
        assert_eq!(Solution::ship_within_days(vec![3,2,2,4,1,4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1,2,3,4,5,6,7,8,9,10], 5), 15);
        assert_eq!(Solution::ship_within_days(vec![1,2,3,1,1], 4), 3);
    }
}
