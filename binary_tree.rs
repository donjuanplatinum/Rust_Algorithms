pub struct TreeNode<T> {
    value: Option<T>,
    left: Option<std::sync::Arc<TreeNode<T>>>,
    right: Option<std::sync::Arc<TreeNode<T>>>,
    count: Option<i32>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode{
            value: Option::None,
            left: Option::None,
            right: Option::None,
            count: Option::None,
        }
    }
}

fn main() {
    
}

