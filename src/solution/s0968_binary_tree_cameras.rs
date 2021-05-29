/**
 * [968] Binary Tree Cameras
 *
 * You are given the root of a binary tree. We install cameras on the tree nodes where each camera at a node can monitor its parent, itself, and its immediate children.
 * Return the minimum number of cameras needed to monitor all nodes of the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_01.png" style="width: 138px; height: 163px;" />
 * Input: root = [0,0,null,0,0]
 * Output: 1
 * Explanation: One camera is enough to monitor all nodes if placed as shown.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_02.png" style="width: 139px; height: 312px;" />
 * Input: root = [0,0,null,0,null,0,null,null,0]
 * Output: 2
 * Explanation: At least two cameras are needed to monitor all nodes of the tree. The above image shows one of the valid configurations of camera placement.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	Node.val == 0
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-cameras/
// discuss: https://leetcode.com/problems/binary-tree-cameras/discuss/?currentPage=1&orderBy=most_votes&query=

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

enum Status {
    Open, // just Open
    Coverd, // covered by other camera
    Camera, // placed a camera
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut num = 0;
        // if root is Open, place a camera
        if matches!(Self::dfs(root, &mut num), Status::Open) {
            num += 1;
        };
        return num;
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, num: &mut i32) -> Status {
        // this magic func return the status of root
        if let Some(p) = root {
            let left = p.borrow_mut().left.take();
            let right = p.borrow_mut().right.take();

            let left_status = Self::dfs(left, num);
            let right_status = Self::dfs(right, num);

            // the initial status of current node is Open
            let mut cur_status = Status::Open;

            // if one of the child has been placed a camera, the current node is covered
            if matches!(left_status, Status::Camera) || matches!(right_status, Status::Camera) {
                cur_status = Status::Coverd;
            }

            if matches!(left_status, Status::Open) || matches!(right_status, Status::Open) {
                *num += 1;
                cur_status = Status::Camera;
            }
            return cur_status;
        }

        Status::Coverd
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_968() {
        // Input: root = [0,0,null,0,null,0,null,null,0]
        // Output: 2
        assert_eq!(Solution::min_camera_cover(tree! [0,0,null,0,null,0,null,null,0]), 2);
        assert_eq!(Solution::min_camera_cover(tree! [0,0,null,0,0]), 1);
    }
}
