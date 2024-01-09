// 1 3 5 6 10 22 30 56 67 99 100 key-> 99
// l             m            r
// arr[m] = 30 < 99 , l = m + 1
// 1 3 5 6 10 22 30 56 67 99 100 key-> 99
//                  l  m      r
// arr[m] = 67 < 99 , l = m + 1
// 1 3 5 6 10 22 30 56 67 99 100 key-> 99
//                        lm  r 
// arr[m] = 99, return Option::<usize>::Some(m)


//and if the key is not exist
// 1 3 5 6 10 22 30 56 67 99 100 key-> 98
// l             m            r
// arr[m] = 30 < 98 , l = m + 1

// 1 3 5 6 10 22 30 56 67 99 100 key-> 98
//                  l  m      r
// arr[m] = 67 < 98 , l = m + 1

// 1 3 5 6 10 22 30 56 67 99 100 key-> 98
//                        lm  r
// arr[m] = 99 > 98 , r = m - 1

// 1 3 5 6 10 22 30 56 67 99 100 key-> 98
//                     r  lm
// and now l > r , so return Option::<usize>::None
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
