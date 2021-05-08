/**
 * [814] Binary Tree Pruning
 *
 * We are given the head node root of a binary tree, where additionally every node's value is either a 0 or a 1.
 * Return the same tree where every subtree (of the given tree) not containing a 1 has been removed.
 * (Recall that the subtree of a node X is X, plus every node that is a descendant of X.)
 * 
 * Example 1:
 * Input: [1,null,0,0,1]
 * Output: [1,null,0,null,1]
 *  
 * Explanation: 
 * Only the red nodes satisfy the property "every subtree not containing a 1".
 * The diagram on the right represents the answer.
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_2.png" style="width:450px" />
 * 
 * Example 2:
 * Input: [1,0,1,0,0,0,1]
 * Output: [1,null,1,null,1]
 * 
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_1.png" style="width:450px" />
 * 
 * Example 3:
 * Input: [1,1,0,1,1,0,1,0]
 * Output: [1,1,0,1,1,null,1]
 * 
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/05/1028.png" style="width:450px" />
 * 
 * Note: 
 * 
 * 	The binary tree will have at most 200 nodes.
 * 	The value of each node will only be 0 or 1.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-pruning/
// discuss: https://leetcode.com/problems/binary-tree-pruning/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(p) = root {
            let left = p.borrow_mut().left.take();
            let left_a = Self::prune_tree(left);

            let right = p.borrow_mut().right.take();
            let right_a = Self::prune_tree(right);

            if left_a.is_none() && right_a.is_none() && p.borrow_mut().val == 0 {
                return None;
            } else {
                p.borrow_mut().left = left_a;
                p.borrow_mut().right = right_a;
            }
            return Some(p);
        }
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_814() {
        assert_eq!(Solution::prune_tree(tree![1,0,1,0,0,0,1]), tree![1,null,1,null,1]);
        assert_eq!(Solution::prune_tree(tree![1,null,0,0,1]), tree![1,null,0,null,1]);
        assert_eq!(Solution::prune_tree(tree![1,1,0,1,1,0,1,0]), tree![1,1,0,1,1,null,1]);
        assert_eq!(Solution::prune_tree(tree![0, 0, 0]), tree![]);
        assert_eq!(Solution::prune_tree(tree![0]), tree![]);
    }
}
