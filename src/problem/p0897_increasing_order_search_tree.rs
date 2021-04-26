/**
 * [897] Increasing Order Search Tree
 *
 * Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/ex1.jpg" style="width: 600px; height: 350px;" />
 * 
 * Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
 * Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
 * 
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/ex2.jpg" style="width: 300px; height: 114px;" />
 * 
 * Input: root = [5,1,7]
 * Output: [1,null,5,null,7]
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	The number of nodes in the given tree will be in the range [1, 100].
 * 	0 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/increasing-order-search-tree/
// discuss: https://leetcode.com/problems/increasing-order-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{borrow::Borrow, rc::Rc};
use std::cell::RefCell;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // travel the root, get the list with increasing order
        let mut dummy_node = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut current_node: Rc<RefCell<TreeNode>> = dummy_node.clone();
        let mut node_list = vec![];
        Self::in_order(root, &mut node_list);

        for node in node_list {
            current_node.borrow_mut().right = Some(node.clone());
            current_node = node;
        }
        let res = dummy_node.borrow_mut().right.take();
        return res;
    }
    
    pub fn in_order(node: Option<Rc<RefCell<TreeNode>>>, node_list: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let lc = node.borrow_mut().left.take();
            let rc = node.borrow_mut().right.take();
            Self::in_order(lc, node_list);
            node_list.push(node);
            Self::in_order(rc, node_list);
        }
    }
}

/* 
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Passed 0ms 1.9mb
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                in_order(left, arr);
                arr.push(node);
                in_order(right, arr);
            }
        }
        let mut arr = Vec::new();
        in_order(root, &mut arr);
        let mut dummy = Rc::new(RefCell::new(TreeNode { val: 0, left: None, right: None }));
        let mut cur = dummy.clone();
        arr.into_iter().for_each(|node| {
            cur.borrow_mut().right = Some(node.clone());
            cur = node;
        });
        let root = dummy.borrow_mut().right.take();
        root
    }
}
*/

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_897() {
        let root = tree![5,3,6,2,4,null,8,1,null,null,null,7,9];
        let res = tree![1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9];
        
        assert_eq!(Solution::increasing_bst(root), res);
    }
}
