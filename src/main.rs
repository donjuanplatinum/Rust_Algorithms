pub mod sort;
pub mod search;
pub mod subarray;
pub mod matrix;

use crate::sort::*;
use crate::matrix::*;
use crate::subarray::*;
fn main() {
    let mut a = [1,2,3,0,5,7,9,8,32,299,100];
    crate::sort::insertion_sort(&mut a);
    println!("a is now {:?}", a);

    let m = (3,[1,2,3,4,5,6,7,8,9]);
    let n = crate::matrix::square_matrix_multiply(m,m);
    println!("matrix n is {:?}", n);

    let q = [-3,1,2,-8,9,0,2,-10,-5,-20,40,-90];
    let c = crate::subarray::merge_subarray(&q);
    println!("the max array is {:?}", c);
    
    let c = crate::search::binary_search(&a,32);
    println!("the c at {:?}", c);
}
