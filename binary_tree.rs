use std::sync::Arc;
pub struct TreeNode<T: std::cmp::PartialEq> {
    value: Option<T>,
    left: Option<std::sync::Arc<TreeNode<T>>>,
    right: Option<std::sync::Arc<TreeNode<T>>>,
    count: Option<i32>,
}

impl<T: std::cmp::PartialEq> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode{
            value: Option::None,
            left: Option::None,
            right: Option::None,
            count: Option::None,
        }
    }
    fn insert(&mut self,value:T) {
        if value <= self.value {
            if self.left != Option::None {
                self.left.insert(value);
            }else {
                self.left = Option::Some(Arc::new(TreeNode::new(value)));
            }
        }
    }
}

fn main() {
    
}

