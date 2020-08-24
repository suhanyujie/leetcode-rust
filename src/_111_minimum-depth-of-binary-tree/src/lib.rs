//! 题目地址：https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

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

    // 将数据装载到树中
    pub fn load_tree(arr: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;
        let mut current_node_op: Option<Rc<RefCell<TreeNode>>> = None;
        for i in 0..arr.len() {
            let x = arr[i];
            match current_node_op {
                Some(ref node_rc) => {
                    // 从根节点开始进行对比，找到合适的位置进行插入
                    let new_node = TreeNode::new(x.clone());
                    let tmp_root = Rc::clone(node_rc);
                    TreeNode::insert(Some(tmp_root), Some(Rc::new(RefCell::new(new_node))));
                }
                None => {
                    // 没有节点值，直接新增节点
                    let new_node = TreeNode::new(x.clone());
                    let new_node_ref = RefCell::new(new_node);
                    let new_node_rc = Rc::new(new_node_ref);
                    current_node_op = Some(Rc::clone(&new_node_rc));
                    root = Some(Rc::clone(&new_node_rc));
                }
            }
        }
        root
    }

    pub fn insert(root: Option<Rc<RefCell<TreeNode>>>, new_node: Option<Rc<RefCell<TreeNode>>>) {
        let mut current_node = root;
        let mut current_node_tmp: Option<Rc<RefCell<TreeNode>>>;
        let mut c1 = 0;
        loop {
            // println!("{:?}", &current_node);
            match current_node {
                Some(ref node_rf) => {
                    let mut node_tr = node_rf.borrow_mut();
                    let new_node_val = if let Some(ref new_node_rf) = new_node {
                        let new_node_tr = (&new_node_rf).borrow();
                        new_node_tr.val
                    } else {
                        panic!("the TreeNode's value is invalid...")
                    };
                    // println!("{}", node_tr.val);
                    if new_node_val > node_tr.val {
                        if node_tr.right == None {
                            node_tr.right = new_node;
                            break;
                        } else {
                            // 获取 right 值的 rc 引用
                            current_node_tmp = TreeNode::get_rc(&(node_tr.right));
                        }
                    } else {
                        if node_tr.left == None {
                            node_tr.left = new_node;
                            break;
                        } else {
                            // 获取 right 值的 rc 引用
                            current_node_tmp = TreeNode::get_rc(&(node_tr.left));
                        }
                    }
                }
                _ => {panic!("something todo")}
            }
            current_node = current_node_tmp;
            c1 += 1;
            if c1 > 3 {
                break;
            }
        }
    }

    pub fn get_rc(rc_rc: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref new_node_rf) = *rc_rc {
            let new_rc = Rc::clone(new_node_rf);
            Some(new_rc)
        } else {
            panic!("the rc's value is invalid...")
        }
    }
}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = TreeNode::load_tree(vec![3, 9, 20, 15, 7]);
        let node = TreeNode::new(3);
        let node = Some(Rc::new(RefCell::new(node)));
        // assert_eq!(1, 0);
        assert_ne!(node, root);
        // assert_eq!(2, Solution::min_depth(root));
    }

    #[test]
    fn insert_1() {
        let node = TreeNode::new(3);
        let root_rc = Rc::new(RefCell::new(node));
        let new1 = TreeNode::new(1);
        let new1_op = Some(Rc::new(RefCell::new(new1)));
        let root_op = Some(Rc::clone(&root_rc));
        TreeNode::insert(root_op, new1_op);
        let new2 = TreeNode::new(2);
        let new2_op = Some(Rc::new(RefCell::new(new2)));
        let root_op = Some(Rc::clone(&root_rc));
        TreeNode::insert(root_op, new2_op);

        println!("{:?}", Rc::clone(&root_rc));
        assert!(false);
    }
}
