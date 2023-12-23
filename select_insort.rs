fn select_insort<T: std::cmp::PartialOrd>(a: &mut [T]) {
    let len = (*a).len();
    for i in 0..len {
        for t in i+1..len {
            if (*a)[t] < a[i] {
                a.swap(t,i);
            }
        }
    }
}

fn main() {
    let mut a: [i32;10] = [1, 9, 7, 20, 100, 60, 3, 8, 2 ,15];
    select_insort::<i32>(&mut a);
    assert_eq!(a, [1,2,3,7,8,9,15,20,60,100])
}
