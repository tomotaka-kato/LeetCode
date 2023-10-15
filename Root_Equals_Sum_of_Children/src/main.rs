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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => {
            let root_node = root.borrow();
            let left_node = root_node.left.as_ref().unwrap().borrow();
            let right_node = root_node.right.as_ref().unwrap().borrow();
            root_node.val == left_node.val + right_node.val
        }
        None => false,
    }
}

fn main() {
    println!("Hello, world!");
}
