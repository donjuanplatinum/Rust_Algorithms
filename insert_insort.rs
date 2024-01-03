
fn insert_insort<T: std::cmp::PartialOrd>(arr: &mut[T]) {
    let len = (*arr).len();
    for index in 1..len {
        let mut i = index ;
        while i > 0 && arr[i - 1] >= arr[i] {   
            (*arr).swap(i,i - 1);
            i -= 1;
        }
    }
}


fn main() {
    let mut arr: [i32;5] = [1, 6, 8, 3, 2];
    let mut arr1: [i32;5] = arr.clone();
    insert_insort::<i32>(&mut arr);
    assert_eq!(arr,[1, 2, 3, 6, 8]);
}
