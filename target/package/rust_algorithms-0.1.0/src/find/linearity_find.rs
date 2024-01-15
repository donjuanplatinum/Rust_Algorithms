// get an array and a number , return the number's position in array 
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


//fn main() {
//    let mut  a: [i32;6] = [1, 3, 6, 10, 1000, 9];
//    let v = 1000;
//    let result = linearity_find(&mut a, v);
//    assert_eq!(result, Option::<usize>::Some(4));
//
