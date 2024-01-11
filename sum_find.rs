fn sum_find<T>(array: &[T], sum: T) -> Option<(usize,usize)> where
    T: PartialOrd + std::ops::Add<Output = T> + Copy, {
    let mut left: usize = 0;
    let mut right: usize = (*array).len() -1 ;
    if ((*array)[right - 1] + (*array)[right]) < sum {
	return Option::<(usize,usize)>::None;
    }
    while left < right {
        let current_sum = array[left] + array[right];
        if current_sum == sum {
            return Some((left, right));
        } else if current_sum < sum {
            left += 1;
        } else {
            right -= 1;
        }
    }
    Option::<(usize,usize)>::None
}

// 1 3 5 7 8 10 90 200 300 key: 91
// l                   r
// 1 + 300 > 91  => right  -= 1
// 1 + 200 > 91  => right  -= 1
// 1 + 90  == 91  => Some((left))

fn main() {
    let a = [1,2,3,4,5,6,7];
    let b = sum_mid_find::<i32>(&a, 8);
    println!("b is {:?}", b);
    let c = [1,2,3,4,5,6,7,8,9];
    let d = sum_mid_find::<i32>(&c, 10);
    println!("d is {:?}", d);

    

}
