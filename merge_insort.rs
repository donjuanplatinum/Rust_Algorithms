fn insert_insort<T: std::cmp::PartialOrd + Clone>(arr: &mut[T]) {
    let len = (*arr).len();
    for index in 0..len {
        let mut i = index ;
        while i >0 && arr[i] < arr[i - 1] {
            (*arr).swap(i,i - 1);
            i = i - 1;
        }
    }
}

fn merge_sort<T: std::cmp::PartialOrd + Clone, const N: usize >(a: &mut[T], p: usize , q: usize, r:usize) ->Vec<T> {
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
    let n = r - p + 1;
    let mut m: Vec<T> = Vec::new();
    for i in p..=r {
        m.push(a[i].clone());
    }
    let mut f = p;
    let mut g = q;
    let b: &[T] = &a[p..=q];
    let c: &[T] = &a[q+1..=r];
    


}

fn main() {
    let  mut a: [i32;15] = [1 ,3 ,2, 6,  4, 6 ,8, 12 , 23, 30 , 132, 2, 3 ,2 ,6];
    let m: Vec<i32> = merge_sort::<i32,15>(&mut a , 0, 7, 14);
    println!("{:?}",m);
}
