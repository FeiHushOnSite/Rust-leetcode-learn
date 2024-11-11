use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[test]
pub fn test_preorder_traversal() {

}

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = VecDeque::from([root]);
    while let Some(x) = stack.pop_back() {
        if let Some(node) = x {
            res.push(node.borrow().val);
            stack.push_back(node.borrow().right.clone());
            stack.push_back(node.borrow().left.clone());
        }
    }
    res
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val:i32,
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
