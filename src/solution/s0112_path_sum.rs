/**
 * [112] Path Sum
 *
 * Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum1.jpg" style="width: 500px; height: 356px;" />
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg" />
 * Input: root = [1,2,3], targetSum = 5
 * Output: false
 * 
 * Example 3:
 * 
 * Input: root = [1,2], targetSum = 0
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-1000 <= Node.val <= 1000
 * 	-1000 <= targetSum <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/path-sum/
// discuss: https://leetcode.com/problems/path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(p) = root {
            let left = p.borrow_mut().left.take();
            let right = p.borrow_mut().right.take();
            
            let val = p.borrow_mut().val;

            if left.is_none() && right.is_none() && val == target_sum {
                return true;
            }
            if Self::has_path_sum(left, target_sum - val) {
                return true;
            }
            if Self::has_path_sum(right, target_sum - val) {
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(Solution::has_path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,null,1], 22), true);
        assert_eq!(Solution::has_path_sum(tree![1, 2, 3], 5), false);
        assert_eq!(Solution::has_path_sum(tree![1, 2], 0), false);
    }
}
