pub struct TreeNode<T> {
    value: Option<T>,
    left: Option<std::sync::Arc<TreeNode<T>>>,
    right: Option<std::sync::Arc<TreeNode<T>>>,
    count: Option<i32>,
}




fn statistic_tree<T: std::cmp::PartialEq + std::clone::Clone>(a : &[T]) ->() {
    let mut root_tree: TreeNode<T> = TreeNode { value: Option::<T>::None ,
                                            left: Option::<std::sync::Arc<TreeNode<T>>>::None,
                                            right: Option::<std::sync::Arc<TreeNode<T>>>::None,
                                            count: Option::<i32>::None };
    for elements in a {
        add_tree(&mut root_tree, elements);
    }


}

fn add_tree<T: std::cmp::PartialEq + std::clone::Clone>(a: &mut TreeNode<T>, b: &T) {
    if (*a).value == Option::None {
        let newtree: TreeNode<T> = TreeNode::<T> {
            value: Option::<T>::Some(b.clone()),
            left: Option::<std::sync::Arc<TreeNode<T>>>::None,
            right: Option::<std::sync::Arc<TreeNode<T>>>::None,
            count: Option::<i32>::Some(1),
        };
    } else  
}
fn main() {
    
}

