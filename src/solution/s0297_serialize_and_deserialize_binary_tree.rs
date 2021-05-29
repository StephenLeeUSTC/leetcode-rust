/**
 * [297] Serialize and Deserialize Binary Tree
 *
 * Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
 * Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
 * Clarification: The input/output format is the same as <a href="/faq/#binary-tree">how LeetCode serializes a binary tree</a>. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg" style="width: 442px; height: 324px;" />
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 * 
 * Example 2:
 * 
 * Input: root = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: root = [1]
 * Output: [1]
 * 
 * Example 4:
 * 
 * Input: root = [1,2]
 * Output: [1,2]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
// discuss: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::VecDeque;
struct Codec {}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        // String::new()
        Codec{}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = "".to_owned();
        let mut q = VecDeque::new();
        q.push_back(root);

        while q.is_empty() == false {
            let node = q.pop_front().unwrap();
            if let Some(p) = node {
                let val = p.try_borrow_mut().unwrap().val;
                res.push_str(&val.to_string());
                let left = p.try_borrow_mut().unwrap().left.take();
                let right = p.try_borrow_mut().unwrap().right.take();
                q.push_back(left);
                q.push_back(right);
            } else {
                res.push_str("null");
            }
            if q.is_empty() == false {
                res.push(',');
            }
        }
        
        res
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vec = data.split(',').collect::<Vec<&str>>();
        let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
        if vec.len() == 0 {
            return None;
        }
        if vec[0].is_none() {
            return None;
        }

        let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
        let mut q = VecDeque::new();
        q.push_back(head.as_ref().unwrap().clone());

        for children in vec[1..].chunks(2) {
            let parent = q.pop_front().unwrap();
            if let Some(v) = children[0] {
                parent.try_borrow_mut().unwrap().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                q.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if children.len() > 1 {
                if let Some(v) = children[1] {
                    parent.try_borrow_mut().unwrap().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                    q.push_back(parent.borrow().right.as_ref().unwrap().clone());
                }
            }
        }

        head
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_297() {
        let root = tree! [];
        let obj = Codec::new();
        let data:String = obj.serialize(root);
        println!("data is {}", data);
        let ans = obj.deserialize(data);
        assert_eq!(ans, tree![]);
    }
}
