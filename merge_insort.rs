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

fn merge_sort<T: std::cmp::PartialOrd, const N: usize >(a: &mut[T;N], p: usize , q: usize, r:usize) ->() {
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
//    let mut m: [T;N] = a;
    let mut f = p;
    let mut g = q;
    for i in p..=r {
        if (*a)[f] < (*a)[g] && f < g {
            m[f] = (*a)[f];
            f = f + 1;
        } else if(*a)[f] > (*a)[g] && g < r {
            m[g] = (*a)[g];
            g = g + 1;
        }
    }

}

fn main() {
    let  mut a: [i32;15] = [1 ,3 ,2, 6,  4, 6 ,8, 12 , 23, 30 , 132, 2, 3 ,2 ,6];
    merge_sort(&mut a , 0, 7, 14);
    println!("{:?}",a);
}
