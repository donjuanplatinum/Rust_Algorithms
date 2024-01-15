pub fn max_subarray<T>(array: &[T]) -> (usize,usize,T) where
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

//fn main() {
//    let array = [1,3,2,-10,3,2,40,-20,-10,20,300,20,1];
//    let p = max_subarray(&array);
//    println!("the max_subarray = {:?}", p);
//}
