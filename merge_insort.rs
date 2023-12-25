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

fn merge_sort<T: std::cmp::PartialOrd>(a: &mut[T], p: usize , q: usize, r:usize) ->() {
    if r < q || q < p {
        eprintln!("must be r > q > p");
    }

//分别排序p..q与q..r
    {
    let b: &mut [T] = &mut a[p..=q];
    insert_insort(b);
    }


    {
    let c: &mut [T] = &mut a[q+1..=r];
    insert_insort(c);
    }


}

fn main() {}
