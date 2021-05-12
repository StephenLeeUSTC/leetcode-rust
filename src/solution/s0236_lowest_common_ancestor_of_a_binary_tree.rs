/**
 * [236] Lowest Common Ancestor of a Binary Tree
 *
 * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
 * Output: 3
 * Explanation: The LCA of nodes 5 and 1 is 3.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
 * Output: 5
 * Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
 * 
 * Example 3:
 * 
 * Input: root = [1,2], p = 1, q = 2
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [2, 10^5].
 * 	-10^9 <= Node.val <= 10^9
 * 	All Node.val are unique.
 * 	p != q
 * 	p and q will exist in the tree.
 * 
 */
pub struct Solution {
}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{borrow::BorrowMut, rc::Rc};
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res : Option<Rc<RefCell<TreeNode>>> = None;
        let p_value = p.unwrap().borrow().val;
        let q_value = q.unwrap().borrow().val;
        let _ = Self::dfs(root, p_value, q_value, &mut res);
        return res;
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32, res: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let left = r.try_borrow_mut().unwrap().left.take();
            let ll = Self::dfs(left, p, q, res);
            let right = r.try_borrow_mut().unwrap().right.take();
            let rr = Self::dfs(right, p, q, res);

            let r_value = r.borrow().val;

            if (ll && rr) || ((r_value == p || r_value == q) && (ll || rr)) {
                *res = Some(Rc::clone(&r));
            }

            return ll || rr || (r_value == p || r_value == q);
        }
        return false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_236() {
    }
}
