use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left = min_depth(node.borrow().left.clone());
            let right = min_depth(node.borrow().right.clone());

            if left == 0 || right == 0 {
                return std::cmp::max(left, right) + 1;
            }

            return std::cmp::min(left, right) + 1;
        }
        None => 0,
    }
}
