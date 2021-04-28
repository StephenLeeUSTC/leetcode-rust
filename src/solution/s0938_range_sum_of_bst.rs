/**
 * [938] Range Sum of BST
 *
 * Given the root node of a binary search tree, return the sum of values of all nodes with a value in the range [low, high].
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst1.jpg" style="width: 400px; height: 222px;" />
 * 
 * Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
 * Output: 32
 * 
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst2.jpg" style="width: 400px; height: 335px;" />
 * 
 * Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
 * Output: 23
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	The number of nodes in the tree is in the range [1, 2 * 10^4].
 * 	1 <= Node.val <= 10^5
 * 	1 <= low <= high <= 10^5
 * 	All Node.val are unique.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/range-sum-of-bst/
// discuss: https://leetcode.com/problems/range-sum-of-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut res = 0;
        Self::inorder(root, low, high, &mut res);
        res
    }

    pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, res: &mut i32) {
        if let Some(p) = root {
            Self::inorder(p.borrow_mut().left.take(), low, high, res);
            let val = p.borrow_mut().val;
            if val >= low && val <= high {
                *res += val;
            }
            Self::inorder(p.borrow_mut().right.take(), low, high, res);
        }

    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_938() {
        assert_eq!(Solution::range_sum_bst(tree![10,5,15,3,7,null,18], 7, 15), 32);
        assert_eq!(Solution::range_sum_bst(tree![10,5,15,3,7,13,18,1,null,6], 6, 10), 23);
    }
}
