/**
 * [986] Interval List Intersections
 *
 * You are given two lists of closed intervals, firstList and secondList, where firstList[i] = [starti, endi] and secondList[j] = [startj, endj]. Each list of intervals is pairwise disjoint and in sorted order.
 * Return the intersection of these two interval lists.
 * A closed interval [a, b] (with a < b) denotes the set of real numbers x with a <= x <= b.
 * The intersection of two closed intervals is a set of real numbers that are either empty or represented as a closed interval. For example, the intersection of [1, 3] and [2, 4] is [2, 3].
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/30/interval1.png" style="width: 700px; height: 194px;" />
 * Input: firstList = [[0,2],[5,10],[13,23],[24,25]], secondList = [[1,5],[8,12],[15,24],[25,26]]
 * Output: [[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]]
 * 
 * Example 2:
 * 
 * Input: firstList = [[1,3],[5,9]], secondList = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: firstList = [], secondList = [[4,8],[10,12]]
 * Output: []
 * 
 * Example 4:
 * 
 * Input: firstList = [[1,7]], secondList = [[3,10]]
 * Output: [[3,7]]
 * 
 *  
 * Constraints:
 * 
 * 	0 <= firstList.length, secondList.length <= 1000
 * 	firstList.length + secondList.length >= 1
 * 	0 <= starti < endi <= 10^9
 * 	endi < starti+1
 * 	0 <= startj < endj <= 10^9 
 * 	endj < startj+1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/interval-list-intersections/
// discuss: https://leetcode.com/problems/interval-list-intersections/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < first_list.len() && j < second_list.len() {
            if first_list[i][1] < second_list[j][0] {
                i += 1;
            } else if second_list[j][1] < first_list[i][0] {
                j += 1;
            } else {
                let left = std::cmp::max(first_list[i][0], second_list[j][0]);
                let right = std::cmp::min(first_list[i][1], second_list[j][1]);
                if first_list[i][1] < second_list[j][1] {
                    i += 1;
                } else if first_list[i][1] > second_list[j][1] {
                    j += 1;
                } else {
                    i += 1;
                    j += 1;
                }
                res.push(vec![left, right]);
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
    fn test_986_0() {
        let first_list = vec![vec![0,2], vec![5,10], vec![13,23], vec![24,25]];
        let second_list = vec![vec![1,5], vec![8,12], vec![15,24], vec![25,26]];
        let expect = vec![vec![1,2], vec![5,5], vec![8,10], vec![15,23], vec![24,24], vec![25,25]];
        assert_eq!(Solution::interval_intersection(first_list, second_list), expect);
    }

    #[test]
    fn test_986_1() {
        let first_list = vec![];
        let second_list = vec![vec![7,12]];
        let expect: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::interval_intersection(first_list, second_list), expect);
    }
}
