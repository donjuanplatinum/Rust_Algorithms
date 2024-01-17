pub fn cross_max_subarray<T>(array: &[T]) -> (usize, usize, T) 
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + Copy + Default
{
    let mid: usize = (array.len() - 1) / 2;

    let mut left_max = T::default();
    let mut right_max = T::default();
    
    let mut left_index: usize = mid;
    let mut right_index: usize = mid;

    let mut sum = T::default();
    for i in (0..=mid).rev() {
        sum += array[i];
        if sum > left_max {
            left_max = sum;
            left_index = i;
        }
    }
  
    sum = T::default();
    for j in mid + 1..array.len() {
        sum += array[j];
        if sum > right_max {
            right_max = sum;
            right_index = j;
        }
    }
  
    (left_index, right_index, left_max + right_max)
}


pub fn merge_subarray<T>(array: &[T]) -> (usize, usize, T)
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + Copy + Default
{
    if array.len() == 1 {
        return (0, 0, array[0]);
    } else {
        let mid = (array.len() - 1) / 2;
        let (left_index, right_index, left_max) = merge_subarray(&array[0..=mid]);
        let (right_start, right_end, right_max) = merge_subarray(&array[mid + 1..]);
        let (cross_start, cross_end, cross_max) = cross_max_subarray(array);
        
        if left_max >= right_max && left_max >= cross_max {
            return (left_index, right_index, left_max);
        } else if right_max >= left_max && right_max >= cross_max {
            return (right_start + mid + 1, right_end + mid + 1, right_max);
        } else {
            return (cross_start, cross_end, cross_max);
        }
    }
}


pub fn rude_ max_subarray<T>(array: &[T]) -> (usize,usize,T) where
    T: std::cmp::PartialEq + std::ops::Add<Output = T> + std::cmp::PartialOrd + std::ops::AddAssign + Copy, {
    let mut max: (usize,usize,T) = (0,0,(*array)[0]);
    for i in 0..(*array).len() - 1 {
	let mut sum: T = (*array)[i] ;
	for j in i + 1..(*array).len() {
	    sum += (*array)[j];
	    if sum > max.2 {
		max.0 = i;
		max.1 = j;
		max.2 = sum;
	    }
	}
    }
    max
}

