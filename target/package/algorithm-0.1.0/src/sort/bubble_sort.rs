pub fn bubble_sort<T: std::cmp::PartialOrd>(a: &mut[T]){
    for i in 0..(*a).len(){
	for j in 0..(*a).len() -i -1 {
	    if (*a)[j] > (*a)[j + 1] {
		(*a).swap(j, j + 1);
	    }
	}
    }

}


//fn main() {
//    let mut a = [5,6,2,3,1,4,10,0,1000,40];
//    bubble_sort::<i32>(&mut a);
//    println!("the a is {:?}", a);
//}
