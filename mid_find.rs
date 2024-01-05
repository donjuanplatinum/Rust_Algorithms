fn mid_find<T: std::cmp::PartialOrd>(arr: &[T], key: T) -> Option<usize> {
    let mut right: usize = (*arr).len();
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



fn main() {

    let arr =[1,2,3,4,5,6,7,8,9,10,100,1000,2323,231412,2131230];
    let index = mid_find::<i32>(&arr, 2323);
    println!("the index is {:?}", index);
}
