use std::cell::RefCell;
use std::rc::Rc;

#[test]
pub fn test_inorder_traversal() {
    let mut res = Vec::new();

}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    in_order(root, &mut ans);
    ans
}

fn in_order(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let node = root.as_ref().unwrap().borrow();
    in_order(node.left, res);
    res.push(node.val);
    in_order(node.right, res);
}

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
            left,
            right,
        }
    }
}
