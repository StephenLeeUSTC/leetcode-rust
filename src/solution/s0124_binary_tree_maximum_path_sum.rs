/**
 * [124] Binary Tree Maximum Path Sum
 *
 * A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
 * The path sum of a path is the sum of the node's values in the path.
 * Given the root of a binary tree, return the maximum path sum of any path.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg" style="width: 322px; height: 182px;" />
 * Input: root = [1,2,3]
 * Output: 6
 * Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg" />
 * Input: root = [-10,9,20,null,null,15,7]
 * Output: 42
 * Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 3 * 10^4].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-maximum-path-sum/
// discuss: https://leetcode.com/problems/binary-tree-maximum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::cmp::max;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_root_leaf = 0;
        let mut max_p_sum = 0;
        Self::post_travel(root, &mut max_root_leaf, &mut max_p_sum);
        max_p_sum
    }

    pub fn post_travel(root: Option<Rc<RefCell<TreeNode>>>, max_root_leaf: &mut i32, max_p_sum:&mut i32) {
        if let Some(node) = root {
            let mut left_root_leaf = 0;
            let mut right_root_leaf = 0;
            let mut left_p_sum = 0;
            let mut right_p_sum = 0;

            let val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            Self::post_travel(left, &mut left_root_leaf, &mut left_p_sum);

            let right = node.borrow_mut().right.take();
            Self::post_travel(right, &mut right_root_leaf, &mut right_p_sum);

            *max_root_leaf = max(max(left_root_leaf, right_root_leaf), 0) + val;
            *max_p_sum = max(max(left_p_sum, right_p_sum), (max(left_root_leaf, 0) + max(right_root_leaf, 0) + val));
        } else {
            *max_root_leaf = 0;
            *max_p_sum = std::i32::MIN;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_124() {
        assert_eq!(Solution::max_path_sum(tree! (1, 2, 3)), 6);
        assert_eq!(Solution::max_path_sum(tree! (-10,9,20,null,null,15,7)), 42);
        assert_eq!(Solution::max_path_sum(tree! (-10)), -10);
        assert_eq!(Solution::max_path_sum(tree! (2, -1)), 2);
        assert_eq!(Solution::max_path_sum(tree! ()), 0);
    }
}
