use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn dfs(root: &mut Rc<RefCell<TreeNode>>, mut sum: &mut i32, depth: i32, max_depth: &mut i32) {
    let mut node = root.borrow_mut();

    if depth > *max_depth {
        *max_depth = depth;
        *sum = node.val;
    } else if depth == *max_depth {
        *sum += node.val;
    }

    if let Some(left) = &mut node.left {
        dfs(left, &mut sum, depth + 1, max_depth);
    } else if let Some(right) = &mut node.right {
        dfs(right, &mut sum, depth + 1, max_depth);
    }
}

pub fn deepest_leaves_sum(mut root: Option<Rc<RefCell<TreeNode>>>)->i32{
    let mut sum = 0;

    if let Some(root) = &mut root {
        dfs(root, &mut sum, 0, &mut 0);
    }

    return sum;
}