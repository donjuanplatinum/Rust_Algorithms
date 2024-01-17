///插入排序
///Θ(n^2)

pub fn insertion_sort<T: std::cmp::PartialOrd>(arr: &mut[T]) {
    
    let len = (*arr).len();
    for index in 1..len {
        let mut i = index ;
        while i > 0 && arr[i - 1] >= arr[i] {   
            (*arr).swap(i,i - 1);
            i -= 1;
        }
    }
}


fn merge<T>(left: Vec<T>, right: Vec<T>) -> Vec<T> where
    T: std::cmp::PartialOrd + Copy,
{

    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    let left_len: usize = left.len();
    let right_len: usize = right.len();
    while left_index < left_len && right_index < right_len {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }

    while left_index < left_len {
        result.push(left[left_index]);
        left_index += 1;
    }

    while right_index < right_len {
        result.push(right[right_index]);
        right_index += 1;
    }

    result
}

///分治排序
///Θ(nlog_{2}n)
pub fn merge_sort<T>(arr: &mut Vec<T>) -> Vec<T> where
    T: std::cmp::PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return arr.clone();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&mut arr[0..mid].to_vec());
    let right = merge_sort(&mut arr[mid..].to_vec());
    merge(left, right)
}

///泡泡排序
///O(n^2)
pub fn bubble_sort<T: std::cmp::PartialOrd>(a: &mut[T]){
    for i in 0..(*a).len(){
	for j in 0..(*a).len() -i -1 {
	    if (*a)[j] > (*a)[j + 1] {
		(*a).swap(j, j + 1);
	    }
	}
    }

}

///选择排序
///O(n^2)
pub fn select_sort<T: std::cmp::PartialOrd>(a: &mut [T]) {
    let len = (*a).len();
    for i in 0..=len {
        for t in i+1..len {
            if (*a)[t] <= a[i] {
                a.swap(t,i);
            }
        }
    }
}


