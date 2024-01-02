fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
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

fn merge_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.clone();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&mut arr[0..mid].to_vec());
    let right = merge_sort(&mut arr[mid..].to_vec());
    merge(left, right)
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);

    let sorted_arr = merge_sort(&mut arr);
    println!("Sorted array: {:?}", sorted_arr);
}
