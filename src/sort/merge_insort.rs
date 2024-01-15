//  0 2 3 1 6 4 9 8
//  0 | 2 | 3 | 1 | 6 | 4 | 9 | 8
//  merge(0,2) merge(3,1) merge(6,4) merge(9,8)
// -> 02 | 13 | 46 | 89 |
//  merge(02,13) merge(46,89)
// -> 0123 | 4689
//  merge(0123,4689)
// -> 01234689
//use rand::{Rng, SeedableRng};
//use rand::distributions::Uniform;
//use rand::rngs::StdRng;


//fn generate_random_array() -> Vec<i32> {
//    let mut rng = StdRng::from_entropy(); // 从系统熵中生成种子
//    let range = Uniform::new(0, 100000); // 随机数范围
//    let mut random_numbers = Vec::with_capacity(1000000);

//    for _ in 0..10000000 {
//        random_numbers.push(rng.sample(range));
//    }

//    random_numbers
//}


pub fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {

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

pub fn merge_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.clone();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&mut arr[0..mid].to_vec());
    let right = merge_sort(&mut arr[mid..].to_vec());
    merge(left, right)
}

//fn main() {
//    let mut random_array = generate_random_array();
//    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
//    println!("Original array: {:?}", arr);

//    let sorted_arr = merge_sort(&mut random_array);
//    println!("Sorted array: {:?}", sorted_arr);
//    println!("sort finished!");
//}
