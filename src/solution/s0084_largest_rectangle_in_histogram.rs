/**
 * [84] Largest Rectangle in Histogram
 *
 * Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg" style="width: 522px; height: 242px;" />
 * Input: heights = [2,1,5,6,2,3]
 * Output: 10
 * Explanation: The above is a histogram where width of each bar is 1.
 * The largest rectangle is shown in the red area, which has an area = 10 units.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg" style="width: 202px; height: 362px;" />
 * Input: heights = [2,4]
 * Output: 4
 * 
 *  
 * Constraints:
 * 
 * 	1 <= heights.length <= 10^5
 * 	0 <= heights[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-rectangle-in-histogram/
// discuss: https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::max;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut res = 0;

        let size = heights.len();
        if size == 1 {
            return heights[0];
        }
        let mut stack = Vec::new();
        for i in 0..size {
            let height = heights[i];
            while stack.is_empty() == false && height < heights[*stack.last().unwrap()] {
                // println!("height is {}, stack is {:?}", height, stack);
                if let Some(j) = stack.pop() {
                    if stack.is_empty() {
                        res = max(heights[j] * i as i32, res);
                    } else {
                        res = max(heights[j] * (i - *stack.last().unwrap() - 1) as i32, res);
                    } 
                }
            }
            stack.push(i);
        }

        // println!("stack is {:?}", stack);

        while stack.is_empty() == false {
            let index = stack.pop().unwrap();
            if stack.is_empty() {
                res = max(size as i32 * heights[index], res);
            } else {
                res = max((size - *stack.last().unwrap() - 1) as i32 * heights[index], res);
            }
        }

        return res;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84_0() {
        let heights = vec![2,1,5,5,2,3];
        assert_eq!(Solution::largest_rectangle_area(heights), 10);
    }

    #[test]
    fn test_84_1() {
        let heights = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(heights), 4);
    }

    #[test]
    fn test_84_2() {
        let heights = vec![5, 4, 1, 2];
        assert_eq!(Solution::largest_rectangle_area(heights), 8);
    }

    #[test]
    fn test_84_3() {
        let heights = vec![4,2,0,3,2,5];
        assert_eq!(Solution::largest_rectangle_area(heights), 6);
    }

   #[test]
    fn test_84_4() {
        let heights = vec![3,6,5,7,4,8,1,0];
        assert_eq!(Solution::largest_rectangle_area(heights), 20);
    }
}
