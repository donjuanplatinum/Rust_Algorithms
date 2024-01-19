fn max_heapify<T: Ord>(arr: &mut [T], n: usize, mut i: usize) {
    loop {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] > arr[largest] {
            largest = left;
        }

        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != i {
            arr.swap(i, largest);
            i = largest;
        } else {
            break;
        }
    }
}

fn build_max_heap<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        max_heapify(arr, n, i);
    }
}
///堆排序
///获取一个可变引用并排序
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    build_max_heap(arr);

    for i in (0..n).rev() {
        arr.swap(0, i);
        max_heapify(arr, i, 0);
    }
}

