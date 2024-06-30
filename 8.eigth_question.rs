//Given a binary tree, implement a function that returns the maximum depth of the tree.
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
  

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root{
            Some(node)=>{
                cmp::max(
                    Solution::max_depth(node.borrow().left.clone()),
                    Solution::max_depth(node.borrow().right.clone())
                )+1
            }
            Node =>0,
        }
    }
}
