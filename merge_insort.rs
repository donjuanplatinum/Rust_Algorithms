fn insert_insort<T: std::cmp::PartialOrd>(arr: &mut[T]) {
    let len = (*arr).len();
    for index in 0..len {
        let mut i = index ;
        while i >0 && arr[i] < arr[i - 1] {
            (*arr).swap(i,i - 1);
            i = i - 1;
        }
    }
}

fn merge_sort<T: std::cmp::PartialOrd>
