pub fn mid_find<T: std::cmp::PartialOrd>(arr: &[T], key: T) -> Option<usize> {
    let mut right: usize = (*arr).len() - 1;
    let mut left: usize = 0;
    while left <= right {
	let mid: usize = (left + right) /2 ;
	if (*arr)[mid] > key {
	    right = mid - 1;
	} else if (*arr)[mid] < key {
	    left = mid + 1;
	} else {
	    return Option::<usize>::Some(mid);
	}
    }
    Option::<usize>::None
}

pub fn sum_find<T>(array: &[T], sum: T) -> Option<(usize,usize)> where
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



pub fn linearity_find<T: std::cmp::PartialEq>(a: &mut [T], v: T) -> Option<usize> {
    let len = (*a).len();
    let mut i =  0;
    while i < len {
        if (*a)[i] != v {
            i = i + 1;
        } else {return Option::Some(i)};
    }
    Option::None
}


