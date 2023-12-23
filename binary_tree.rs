pub struct TreeNode<T> {
    value: Option<T>,
    left: Option<std::sync::Arc<TreeNode<T>>>,
    right: Option<std::sync::Arc<TreeNode<T>>>,
    count: i32,
}


pub trait BinaryTree {
    fn new<T>() -> TreeNode<T>;
    fn get_left<T>(&self) -> TreeNode<T>;
    fn get_right<T>(&self) -> TreeNode<T>;
    fn get_parent<T>(&self) -> TreeNode<T>;
}

fn statistic_tree<T: std::cmp::PartialEq>(a : &[T]) ->() {
    let root_tree: TreeNode<T>;
    

}
fn main() {
    
}

