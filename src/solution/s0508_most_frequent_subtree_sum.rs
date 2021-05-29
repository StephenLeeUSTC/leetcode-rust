/**
 * [508] Most Frequent Subtree Sum
 *
 * Given the root of a binary tree, return the most frequent subtree sum. If there is a tie, return all the values with the highest frequency in any order.
 * The subtree sum of a node is defined as the sum of all the node values formed by the subtree rooted at that node (including the node itself).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/freq1-tree.jpg" style="width: 207px; height: 183px;" />
 * Input: root = [5,2,-3]
 * Output: [2,-3,4]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/freq2-tree.jpg" style="width: 207px; height: 183px;" />
 * Input: root = [5,2,-5]
 * Output: [2]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-10^5 <= Node.val <= 10^5
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/most-frequent-subtree-sum/
// discuss: https://leetcode.com/problems/most-frequent-subtree-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = HashMap::new();
        let _ = Self::get_sum(root, &mut map);
        let mut res = Self::parse_map(map);
        res
    }

    pub fn get_sum(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(v) = root {
            let left = v.borrow_mut().left.take();
            let right = v.borrow_mut().right.take();
            let val = v.borrow().val;
            let l_sum = Self::get_sum(left, map);
            let r_sum = Self::get_sum(right, map);
            // println!("val is {}, lsum is {}, rsum is {}", val, l_sum, r_sum);
            let key = l_sum + r_sum + val;
            map.entry(key).and_modify(|e| { *e += 1}).or_insert(1);
            return key;
        }
        0
    }

    pub fn parse_map(map: HashMap<i32, i32>) -> Vec<i32> {
        // println!("map is {:?}", map);
        let mut res = vec![];
        let mut max_value = -1;
        for kv in map.iter() {
            max_value = max(max_value, *kv.1);
        }

        for kv in map.iter() {
            if *kv.1 == max_value {
                res.push(*kv.0);
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
    fn test_508() {
        // Input: root = [5,2,-3]
        // Output: [2,-3,4]
        assert_eq!(Solution::find_frequent_tree_sum(tree! (5, 2, -3)), vec![-3, 2, 4]);
        assert_eq!(Solution::find_frequent_tree_sum(tree! (5, 2, -5)), vec![2]);
    }
}
